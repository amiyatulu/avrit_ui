import React, { useState, useEffect } from "react"
import { Route, Switch } from "react-router-dom"
import CreateProfile from "./profile/CreateProfile"
import { NearContext } from "./context/NearContext"
import Nav from "./components/Nav"
import ViewProfile from "./profile/ViewProfile"
import UpdateProfile from "./profile/UpdateProfile"
import CreateProduct from "./products/CreateProduct"
import GetProducts from "./products/GetProducts"
import ProductById from "./products/ProductById"
import AvritToken from "./profile/AvritToken"
import CreateReviewEvidence from "./reviews/CreateReviewEvidence"
import CreateReviewStake from "./stakes/CreateReviewStake"
import GetReviewStake from "./stakes/GetReviewStake"
import ApplyJuryStake from "./schelling/ApplyJuryStake"
import GetJuryStake from "./schelling/GetJuryStake"

function App(props) {
  const [login, setLogin] = useState(false)
  const [speech, setSpeech] = useState(null)
  // constructor(props) {
  //   super(props)
  //   this.state = {
  //     login: false,
  //     speech: null,
  //   }
  //   this.signedInFlow = this.signedInFlow.bind(this)
  //   this.requestSignIn = this.requestSignIn.bind(this)
  //   this.requestSignOut = this.requestSignOut.bind(this)
  //   this.signedOutFlow = this.signedOutFlow.bind(this)
  // }

  useEffect(() => {
    async function login() {
      let loggedIn = props.wallet.isSignedIn()
      if (loggedIn) {
        signedInFlow()
      } else {
        signedOutFlow()
      }
    }
    login()
    console.log("Main use effect")
  }, [props])

  // componentDidMount() {
  //   let loggedIn = this.props.wallet.isSignedIn()
  //   if (loggedIn) {
  //     this.signedInFlow()
  //   } else {
  //     this.signedOutFlow()
  //   }
  // }

  async function signedInFlow() {
    console.log("come in sign in flow")
    setLogin(true)
    const accountId = await props.wallet.getAccountId()
    if (window.location.search.includes("account_id")) {
      window.location.replace(window.location.origin + window.location.pathname)
    }
    // await this.welcome();
  }

  // async welcome() {
  //   const response = await this.props.contract.welcome({ account_id: accountId });
  //   this.setState({speech: response.text});
  // }

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

  // async changeGreeting() {
  //   await this.props.contract.set_greeting({ message: 'howdy' });
  //   await this.welcome();
  // }

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
    <NearContext.Provider value={{ nearvar: props }}>
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
          </Switch>
        </section>
      </React.Fragment>
    </NearContext.Provider>
  )
}

export default App
