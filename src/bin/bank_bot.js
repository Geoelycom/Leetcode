// You've been asked to program a bot for a popular bank that will automate the management of incoming requests. Every request has its own timestamp in seconds, and it is guaranteed that all requests come sequentially, i.e. the timestamp is strictly increasing. There are two types of incoming requests:

// deposit <timestamp> <holder_id> <amount> — request to deposit <amount> amount of money in the <holder_id> account;
// withdraw <timestamp> <holder_id> <amount> — request to withdraw <amount> amount of money from the <holder_id> account. As a bonus, bank also provides a cashback policy — 2% of the withdrawn amount rounded down to the nearest integer will be returned to the account exactly 24 hours after the request timestamp. If the cashbask and deposit/withdrawal happen at the same timestamp, assume cashback happens earlier.
// Your system should also handle invalid requests. There are two types of invalid requests:

// invalid account number;
// withdrawal of a larger amount of money than is currently available.
// For the given list of initial balances and requests, return the state of balances straight after the last request has been processed, or an array of a single element [-<request_id>] (please note the minus sign), where <request_id> is the 1-based index of the first invalid request. Note that cashback requests which haven't happened before the last request should be ignored.

// Example

// Example 1
// For balances = [1000, 1500] and requests = ["withdraw 1613327630 2 480", "withdraw 1613327644 2 800", "withdraw 1614105244 1 100", "deposit 1614108844 2 200", "withdraw 1614108845 2 150"], the output should be solution(balances, requests) = [900, 295].

// Explanation
// Here are the states of balances after each request:

// initially: [1000, 1500];
// "withdraw 1613327630 2 480": [1000, 1020];
// "withdraw 1613327644 2 800": [1000, 220];
// At 1613414030 the 2nd account will receive the cashback of 480 * 0.02 = 9.6, which is rounded down to 9: [1000, 229];
// At 1613414044 the 2nd account will receive the cashback of 800 * 0.02 = 16: [1000, 245];
// "withdraw 1614105244 1 100": [900, 245];
// "deposit 1614108844 2 200": [900, 445];
// "withdraw 1614108845 2 150": [900, 295], which is the answer.
// Cashbacks for the last two withdrawals happen at 1614191644 and 1614195245, which is after the last request timestamp 1614108845, so they are ignored.
// Example 2
// For balances = [20, 1000, 500, 40, 90] and requests = ["deposit 1613327630 3 400", "withdraw 1613327635 1 20", "withdraw 1613327651 1 50", "deposit 1613327655 1 50"], the output should be solution(balances, requests) = [-3].

// Explanation
// Here are the states of balances after each request:

// initially: [20, 1000, 500, 40, 90];
// "deposit 1613327630 3 400": [20, 1000, 900, 40, 90];
// "withdraw 1613327635 1 20": [0, 1000, 900, 40, 90];
// "withdraw 1613327651 1 50": it is not possible to withdraw 50 from the 1st account, so the request is invalid.
// the rest of the requests are not processed
// Input/Output

// [execution time limit] 4 seconds (js)

// [memory limit] 1 GB

// [input] array.integer balances

// Array of integers, where balances[i] is the amount of money in the (i + 1)th account.

// Guaranteed constraints:
// 1 ≤ balances.length ≤ 100,
// 0 ≤ balances[i] ≤ 105.

// [input] array.string requests

// Array of requests in the order they should be processed. Each request is guaranteed to be in the format described above. It is guaranteed that requests come sequentially, i.e. the timestamp strictly increases.

// Guaranteed constraints:
// 1 ≤ requests.length ≤ 100.

// [output] array.integer

// The balances after executing all of the requests or array with a single integer - the index of the first invalid request preceded by -.

// [JavaScript] Syntax Tips

// // Prints help message to the console
// // Returns a string
// function helloWorld(name) {
//     console.log("This prints to the console when you Run Tests");
//     return "Hello, " + name;
// }

function solution(balances, requests) {
  const cashbackQueue = []; // Store future cashback events: [timestamp, holderId, cashbackAmount]
  let lastTimestamp = 0;

  for (let i = 0; i < requests.length; i++) {
      const requestId = i + 1;
      const parts = requests[i].split(" ");
      const type = parts[0];
      const timestamp = parseInt(parts[1]);
      const holderId = parseInt(parts[2]) - 1;
      const amount = parseInt(parts[3]);

      // Apply all pending cashback events up to current timestamp
      while (cashbackQueue.length && cashbackQueue[0][0] <= timestamp) {
          const [cbTimestamp, cbHolderId, cbAmount] = cashbackQueue.shift();
          if (cbHolderId >= 0 && cbHolderId < balances.length) {
              balances[cbHolderId] += cbAmount;
          }
      }

      // Validation: check account exists
      if (holderId < 0 || holderId >= balances.length) {
          return [-requestId];
      }

      if (type === "deposit") {
          balances[holderId] += amount;
      } else if (type === "withdraw") {
          if (balances[holderId] < amount) {
              return [-requestId];
          }
          balances[holderId] -= amount;

          const cashbackAmount = Math.floor(amount * 0.02);
          const cashbackTime = timestamp + 86400; // 24 hours = 86400 seconds
          cashbackQueue.push([cashbackTime, holderId, cashbackAmount]);
      }

      lastTimestamp = timestamp;
  }

  // Apply any remaining cashback that should have occurred before last timestamp
  cashbackQueue.sort((a, b) => a[0] - b[0]);
  for (const [cbTimestamp, cbHolderId, cbAmount] of cashbackQueue) {
      if (cbTimestamp <= lastTimestamp) {
          balances[cbHolderId] += cbAmount;
      }
  }

  return balances;
}


console.log(solution(
  [1000, 1500],
  ["withdraw 1613327630 2 480", "withdraw 1613327644 2 800", "withdraw 1614105244 1 100", "deposit 1614108844 2 200", "withdraw 1614108845 2 150"]
)); // Output: [900, 295]

console.log(solution(
  [20, 1000, 500, 40, 90],
  ["deposit 1613327630 3 400", "withdraw 1613327635 1 20", "withdraw 1613327651 1 50", "deposit 1613327655 1 50"]
)); // Output: [-3]


