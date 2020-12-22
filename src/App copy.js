import React, { Component } from "react"
import { Route, Switch } from "react-router-dom"
import CreateProfile from "./profile/CreateProfile"
import { NearContext } from "./context/NearContext"
import Nav from "./components/Nav"
import ViewProfile from "./profile/ViewProfile"
import UpdateProfile from "./profile/UpdateProfile"
import CreateProduct from "./products/CreateProduct"
import GetProducts from "./products/GetProducts"
import ProductById from "./products/ProductById"
import AvritToken from './profile/AvritToken';
import CreateReviewEvidence from './reviews/CreateReviewEvidence';
import CreateReviewStake from './stakes/CreateReviewStake';
import GetReviewStake from './stakes/GetReviewStake';
import ApplyJuryStake from './schelling/ApplyJuryStake';
import GetJuryStake from './schelling/GetJuryStake';

class App extends Component {
  constructor(props) {
    super(props)
    this.state = {
      login: false,
      speech: null,
    }
    this.signedInFlow = this.signedInFlow.bind(this)
    this.requestSignIn = this.requestSignIn.bind(this)
    this.requestSignOut = this.requestSignOut.bind(this)
    this.signedOutFlow = this.signedOutFlow.bind(this)
  }

  componentDidMount() {
    let loggedIn = this.props.wallet.isSignedIn()
    if (loggedIn) {
      this.signedInFlow()
    } else {
      this.signedOutFlow()
    }
  }

  async signedInFlow() {
    console.log("come in sign in flow")
    this.setState({
      login: true,
    })
    const accountId = await this.props.wallet.getAccountId()
    if (window.location.search.includes("account_id")) {
      window.location.replace(window.location.origin + window.location.pathname)
    }
    // await this.welcome();
  }

  // async welcome() {
  //   const response = await this.props.contract.welcome({ account_id: accountId });
  //   this.setState({speech: response.text});
  // }

  async requestSignIn() {
    const appTitle = "NEAR React template"
    await this.props.wallet.requestSignIn(
      window.nearConfig.contractName,
      appTitle
    )
  }

  requestSignOut() {
    this.props.wallet.signOut()
    localStorage.removeItem("my-profile")
    setTimeout(this.signedOutFlow, 500)
    console.log("after sign out", this.props.wallet.isSignedIn())
  }

  // async changeGreeting() {
  //   await this.props.contract.set_greeting({ message: 'howdy' });
  //   await this.welcome();
  // }

  signedOutFlow() {
    if (window.location.search.includes("account_id")) {
      window.location.replace(window.location.origin + window.location.pathname)
    }
    this.setState({
      login: false,
      speech: null,
    })
  }

  render() {
    let style = {
      fontSize: "1.5rem",
      color: "#0072CE",
      textShadow: "1px 1px #D1CCBD",
    }
    return (
      <React.Fragment>
        {this.state.login ? (
          <NearContext.Provider value={this.props}>
            <Nav onClick={this.requestSignOut} login={this.state.login} />{" "}
          </NearContext.Provider>
        ) : (
          <Nav onClick={this.requestSignIn} login={this.state.login} />
        )}
        <section className="page-section">
          <Switch>
            <NearContext.Provider value={this.props}>
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
            </NearContext.Provider>
          </Switch>
        </section>
      </React.Fragment>
    )
  }
}

export default App
