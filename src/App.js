import React, { useState, useEffect, useCallback } from "react"
import { Route, Switch } from "react-router-dom"
import CreateProfile from "./components/profile/CreateProfile"
import { NearContext } from "./commons/context/NearContext"
import Nav from "./components/Nav"
import ViewProfile from "./components/profile/ViewProfile"
import UpdateProfile from "./components/profile/UpdateProfile"
import CreateProduct from "./components/products/CreateProduct"
import GetProducts from "./components/products/GetProducts"
import ProductById from "./components/products/ProductById"
import AvritToken from "./components/profile/AvritToken"
import CreateReviewEvidence from "./components/reviews/CreateReviewEvidence"
import CreateReviewStake from "./components/stakes/CreateReviewStake"
import GetReviewStake from "./components/stakes/GetReviewStake"
import ApplyJuryStake from "./components/schelling/ApplyJuryStake"
import GetJuryStake from "./components/schelling/GetJuryStake"
import JuryApplyTime from "./components/stakes/JuryApplyTime"
import CommitVote from "./components/schelling/CommitVote"
import CommitSubmitted from "./components/schelling/CommitSubmitted"

function App(props) {
  const [login, setLogin] = useState(false)
  const [speech, setSpeech] = useState(null)
  const [balance, setBalance] = useState(null)
  const [balanceError, setBalanceError] = useState(null)
  const [userId, setUserId] = useState(null)
  // console.log(balance)
  // console.log(userId)

  async function fetchProfile() {
    let data
    try {
      data = await props.contract.get_balance({
        owner_id: props.wallet.getAccountId(),
      })
      // console.log(props.wallet.getAccountId())
      // console.log(data)
      // console.log("fetchProfile")
      setBalance(data)
    } catch (e) {
      console.log(e.message)
      const failedtofetch = e.message
      setBalanceError(failedtofetch)
    }
  }

  async function fetchUserId() {
    let userid
    try {
      userid = await props.contract.get_user_id_js({
        account_id: props.wallet.getAccountId(),
      })
      setUserId(parseInt(userid))
      // console.log(props.wallet.getAccountId())
      // console.log(data)
      // console.log("fetchuserid")
    } catch (e) {
      console.log(e.message)
      const failedtofetch = e.message
    }
  }

  const callUserId = useCallback(async () => {
    fetchUserId()
  }, [])

  const reloadBalance = useCallback(async () => {
    fetchProfile()
  }, [])

  useEffect(() => {
    async function login() {
      let loggedIn = props.wallet.isSignedIn()
      if (loggedIn) {
        signedInFlow()
        reloadBalance()
        callUserId()
      } else {
        signedOutFlow()
      }
    }
    login()
    console.log("Main use effect")
  }, [props])

  async function signedInFlow() {
    console.log("come in sign in flow")
    setLogin(true)
    const accountId = await props.wallet.getAccountId()
    if (window.location.search.includes("account_id")) {
      window.location.replace(window.location.origin + window.location.pathname)
    }
  }

  async function requestSignIn() {
    const appTitle = "NEAR React template"
    await props.wallet.requestSignIn(window.nearConfig.contractName, appTitle)
  }

  function requestSignOut() {
    props.wallet.signOut()
    localStorage.removeItem("my-profile")
    setTimeout(signedOutFlow, 500)
    console.log("after sign out", props.wallet.isSignedIn())
  }

  function signedOutFlow() {
    if (window.location.search.includes("account_id")) {
      window.location.replace(window.location.origin + window.location.pathname)
    }
    setLogin(false)
    setSpeech(null)
  }

  let style = {
    fontSize: "1.5rem",
    color: "#0072CE",
    textShadow: "1px 1px #D1CCBD",
  }
  return (
    <NearContext.Provider
      value={{ nearvar: props, reloadBalance, balance, balanceError, userId }}
    >
      <React.Fragment>
        {login ? (
          <Nav onClick={requestSignOut} login={login} />
        ) : (
          <Nav onClick={requestSignIn} login={login} />
        )}
        <section className="page-section">
          <Switch>
            <Route path="/createprofile" component={CreateProfile} />
            <Route path="/profile" component={ViewProfile} />
            <Route path="/updateprofile" component={UpdateProfile} />
            <Route path="/createproduct" component={CreateProduct} />
            <Route path="/myproducts" component={GetProducts} />
            <Route path="/product/:id" component={ProductById} />
            <Route path="/balance" component={AvritToken} />
            <Route path="/createreview/:pid" component={CreateReviewEvidence} />
            <Route path="/reviewstake/:rid" component={CreateReviewStake} />
            {/* <Route path="/getreviewstake/:rid" component={GetReviewStake} /> */}
            <Route path="/applyjury/:rid" component={ApplyJuryStake} />
            <Route path="/getjurystake/:rid/:userId" component={GetJuryStake} />
            {/* <Route path="/juryapplytime/:rid" component={JuryApplyTime} /> Remove it later*/}
            <Route path="/commitvote/:rid" component={CommitVote} />
            <Route path="/commitsubmitted" component={CommitSubmitted} />
          </Switch>
        </section>
      </React.Fragment>
    </NearContext.Provider>
  )
}

export default App
