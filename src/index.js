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
  // let acct = await new nearlib.Account(window.near.connection, window.accountId)
  window.contract = await new nearlib.Contract(window.walletAccount.account(), window.nearConfig.contractName, {
    // View methods are read only. They don't modify the state, but usually return some value.
    viewMethods: ["get_user_id_js","get_product_js", "ft_balance_of", "get_review_ids_by_product_id", "get_review_js", "get_products_of_user_id_js", "get_review_bounty_js", "get_juror_stakes_js", "get_jury_application_phase_time", "get_jury_application_start_time_js",  "get_juror_selection_time_js", "contract.get_commit_phase_time", "get_commit_phase_time", "get_reveal_phase_time","number_of_staked_jury", "can_juror_vote_bool", "get_selected_juror_count", "get_user_profile_js", "can_juror_unstake_bool", "can_juror_reveal", "get_true_count_js", "get_false_count_js", "if_juror_will_get_incentives", "if_review_get_incentives_bool", "check_product_will_get_incentives_bool", "get_product_crowdfunding_js", "get_final_product_id", "required_deposit", "get_token_sold", "get_phase_available_tokens", "get_username", "get_min_jury_stake", "get_min_review_bounty", "get_nft_price_js", "get_total_nft_count_js", "last_ten_tokens_for_owner", "get_owner_incentives", "p_get_jury_application_phase_time", "p_get_jury_application_start_time_js", "p_get_juror_selection_time_js", "p_get_commit_phase_time", "p_get_reveal_phase_time", "get_product_bounty_js", "p_get_juror_stakes_js", "p_number_of_staked_jury", "p_get_min_jury_stake", "p_can_juror_vote_bool", "p_get_selected_juror_count", "p_can_juror_unstake_bool", "p_get_true_count_js", "p_get_false_count_js", "p_can_juror_reveal", "p_if_juror_will_get_incentives", "p_if_product_get_incentives_bool"],
    // Change methods can modify the state. But you don't receive the returned value when called.
    changeMethods: ["create_profile", "update_profile" , "get_profile_hash", 'create_product', "update_product", "create_review", "add_review_bounty", "apply_jurors", "commit_vote", "update_review", "draw_jurors", "unstaking_non_selected_juror", "reveal_vote", "incentives_distribution", "incentive_distribution_reviewer", "incentive_distribution_product", "add_product_bounty", "add_product_crowdfunding", "buy_tokens", "setup_nft_price_and_token_count", "buy_nft", "withdraw_product_owner_incentives", "buy_nft2", "p_apply_jurors", "p_draw_jurors", "p_commit_vote", "p_unstaking_non_selected_juror", "p_reveal_vote", "p_incentives_distribution", "p_incentive_distribution_product"],
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
