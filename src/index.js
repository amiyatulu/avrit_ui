import React from "react"
import ReactDOM from "react-dom"
import "./index.css"
import App from "./App"
import * as serviceWorker from "./serviceWorker"
import getConfig from "./config/config.js"
import * as nearlib from "near-api-js"
import { HashRouter } from "react-router-dom"

// Initializing contract
async function initContract() {
  console.log(process.env.NODE_ENV)
  window.nearConfig = getConfig(process.env.NODE_ENV || "development")
  console.log("nearConfig", window.nearConfig)

  // Initializing connection to the NEAR DevNet.
  window.near = await nearlib.connect(Object.assign({ deps: { keyStore: new nearlib.keyStores.BrowserLocalStorageKeyStore() } }, window.nearConfig))

  // Needed to access wallet login
  window.walletAccount = new nearlib.WalletAccount(window.near)

  // Getting the Account ID. If unauthorized yet, it's just empty string.
  window.accountId = window.walletAccount.getAccountId()

  // Initializing our contract APIs by contract name and configuration.
  let acct = await new nearlib.Account(window.near.connection, window.accountId)
  window.contract = await new nearlib.Contract(acct, window.nearConfig.contractName, {
    // View methods are read only. They don't modify the state, but usually return some value.
    viewMethods: ["get_user_id_js","get_product", "get_balance", "get_review_ids_by_product_id", "get_review", "get_review_bounty_js", "get_juror_stakes_js"],
    // Change methods can modify the state. But you don't receive the returned value when called.
    changeMethods: ["create_profile", "update_profile" , "get_profile_hash", 'create_product', "get_products_of_user", "update_product", "create_review", "add_review_bounty", "apply_jurors"],
    // Sender is the account ID to initialize transactions.
    sender: window.accountId,
  })
}

window.nearInitPromise = initContract()
  .then(() => {
    ReactDOM.render(
      <React.StrictMode>
        <HashRouter>
          <App contract={window.contract} wallet={window.walletAccount} />
        </HashRouter>
      </React.StrictMode>,
      document.getElementById("root")
    )
  })
  .catch(console.error)

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister()
