import assert from 'assert';
import { Wallet, EsploraClient, ChangeSet } from 'bitcoindevkit';

const externalDescriptor = "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/0/*)#z3x5097m";
const internalDescriptor = "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/1/*)#n9r4jswr";


async function run() {    
  let wallet;
  let client = new EsploraClient("https://mutinynet.com/api");
  console.log("Creating new wallet");
  wallet = Wallet.create(
      "signet",
      externalDescriptor,
      internalDescriptor
  );

  console.log("Performing Full Scan...");
  const full_scan_request = wallet.start_full_scan();
  let update = await client.full_scan(full_scan_request, 1);
  wallet.apply_update(update);

  const walletDataString = wallet.take_staged().to_json();
  console.log("Scan Results:", walletDataString);

  console.log("Deleting wallet");
  wallet = null;

  console.log("Loading wallet");
  let changeSet = ChangeSet.from_json(walletDataString);
  wallet = Wallet.load(
      changeSet,
      externalDescriptor,
      internalDescriptor
  );

  console.log("Syncing...");
  const sync_request = wallet.start_sync_with_revealed_spks();
  update = await client.sync(sync_request, 1);
  wallet.apply_update(update);  

  // Test balance
  console.log("Balance:", wallet.balance().confirmed.to_sat());
  
  // Test address generation
  console.log("New address:", wallet.reveal_next_address().address);

  const updateChangeSet = wallet.take_staged();
  assert.equal(!!updateChangeSet, true); // updated by address generation (if not sync)

  // handle merging
  console.log("Update:", updateChangeSet.to_json());
  let currentChangeSet = ChangeSet.from_json(walletDataString);
  console.log("Current:", currentChangeSet.to_json());
  currentChangeSet.merge(updateChangeSet);
  console.log("Merged:", currentChangeSet.to_json());
}

// Test case
try {
  await run();
  console.log('✅ Test passed!');
} catch (err) {
    console.error('❌ Test failed:', err);
} 