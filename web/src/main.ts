// The page of the dapp. Everything specific to the contracts lives in this file; casper.ts and
// network.ts stay as they are.
import { AccountInfo, DailyAuctionErrors, DailyAuctionWasmClient, TransactionResult, U256 } from "contract-client";
import { Casper, currentAccount, formatCspr, parseCspr, prepareRead, readOr, send, short, startCasper } from "./casper";
import { NETWORK_NAME, accountUrl, contractUrl, transactionUrl } from "./network";

const $ = <T extends HTMLElement>(id: string) => document.getElementById(id) as T;

let casper: Casper;
let contract: DailyAuctionWasmClient;
let roundEndsAt = 0n;

function tick() {
  if (!roundEndsAt) return;
  const s = Math.floor((Number(roundEndsAt) - Date.now()) / 1000);
  const pad = (n: number) => String(n).padStart(2, "0");
  $("countdown").textContent =
    s <= 0 ? "finished" : `${pad(Math.floor(s / 3600))}:${pad(Math.floor((s % 3600) / 60))}:${pad(s % 60)}`;
}

function setStatus(text: string, kind: "" | "ok" | "err" = "", link?: string) {
  const status = $("status");
  status.className = kind;
  status.textContent = text;
  if (link) {
    const a = document.createElement("a");
    a.href = link;
    a.target = "_blank";
    a.rel = "noopener noreferrer";
    a.textContent = " View in the explorer";
    status.append(a);
  }
}

/** Reads the contract and draws it. Called at start and after every successful transaction. */
async function refresh() {
  prepareRead();
  const round = await contract.currentRound();
  roundEndsAt = await contract.roundEndsAt();
  const highestBid = await contract.highestBid();
  const highestBidder = await contract.highestBidder();
  $("round").textContent = `#${round}`;
  $("highest-bid").textContent = highestBidder ? formatCspr(highestBid) : "no bids yet";
  $("highest-bidder").textContent = highestBidder ? short(highestBidder.value) : "—";
  $("min-bid").textContent = formatCspr(await contract.minBid());

  const history = $("history");
  history.replaceChildren();
  const first = round > 5n ? round - 5n : 0n;
  for (let r = round - 1n; r >= first; r--) {
    const owner = await readOr(undefined, () => contract.ownerOf(U256.fromBigInt(r)));
    const item = document.createElement("li");
    item.innerHTML = `<span>NFT #${r}</span><span></span>`;
    item.lastElementChild!.textContent = owner ? short(owner.value) : "not minted";
    history.append(item);
  }
  if (!history.children.length) history.innerHTML = `<li class="muted">The first round is still open.</li>`;
}

function onAccount(account: AccountInfo | null) {
  const link = $<HTMLAnchorElement>("account");
  link.textContent = account ? short(account.publicKey) : "";
  if (account) link.href = accountUrl(account.publicKey);
  $("connect").hidden = !!account;
  $("disconnect").hidden = !account;
}

/** Turns a failed transaction into a sentence. Map the contract's error codes here. */
function describeError(result: TransactionResult): string {
  if (result.errorCode === DailyAuctionErrors.BidTooLow) return "Your bid must be higher than the current highest bid.";
  if (result.errorCode === DailyAuctionErrors.BidBelowMinimum) return "Your bid is below the minimum bid.";
  return result.error ?? "The transaction failed.";
}

function onTransaction(state: "sent" | "done" | "failed", result: TransactionResult) {
  const link = result.txHash ? transactionUrl(result.txHash) : undefined;
  if (state === "sent") setStatus("Transaction sent, waiting for the network…", "", link);
  if (state === "failed") setStatus(describeError(result), "err", link);
  if (state === "done") {
    setStatus("Done.", "ok", link);
    refresh().catch(console.error);
  }
}

/**
 * Runs a state-changing call, e.g. `transact(() => send(10, () => contract.doSomething(arg)))`.
 * Asks the user to connect first; the wallet then shows the transaction for approval.
 */
export async function transact(call: () => Promise<TransactionResult>) {
  if (!currentAccount()) {
    await casper.client.signIn();
    return;
  }
  try {
    setStatus("Confirm the transaction in your wallet…");
    const result = await call();
    if (result.isCancelled) setStatus("Cancelled.");
  } catch (e) {
    setStatus(String(e), "err");
  }
}

async function main() {
  $("network").textContent = NETWORK_NAME;
  casper = await startCasper("Daily Auction", { onAccount, onTransaction });
  const info = casper.contracts.get("DailyAuction");
  contract = new DailyAuctionWasmClient(casper.client, info.address);

  const link = $<HTMLAnchorElement>("contract-link");
  link.textContent = short(info.package_hash);
  link.href = contractUrl(info.package_hash);

  $("connect").addEventListener("click", () => casper.client.signIn());
  $("disconnect").addEventListener("click", () => casper.client.signOut());
  $("bid-form").addEventListener("submit", (event) => {
    event.preventDefault();
    const amount = parseCspr($<HTMLInputElement>("amount").value);
    if (!amount) return setStatus("Enter the amount as a number, for example 12.5", "err");
    transact(() => send(15, () => contract.bid(amount)));
  });
  $("settle").addEventListener("click", () => transact(() => send(20, () => contract.settle())));
  setInterval(tick, 1000);
  setInterval(() => refresh().catch(console.error), 30_000);

  (window as any).dbg = { contract, U256, casper };
  await refresh();
}

main().catch((e) => {
  console.error(e);
  setStatus(`Could not start: ${e}`, "err");
});
