import React, { Component } from "react"
import { Route, Switch } from "react-router-dom"
import CreateProfile from "./profile/CreateProfile"
import { NearContext } from "./context/NearContext"
import Nav from "./components/Nav"

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
    await this.props.wallet.requestSignIn(window.nearConfig.contractName, appTitle)
  }

  requestSignOut() {
    this.props.wallet.signOut()
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
        {this.state.login ? <Nav onClick={this.requestSignOut} login={this.state.login}/> : <Nav onClick={this.requestSignIn} login={this.state.login}/>}
        <section className="page-section">
          <Switch>
            <NearContext.Provider value={this.props}>
              <Route path="/" exact component={CreateProfile} />
            </NearContext.Provider>
          </Switch>
        </section>
      </React.Fragment>
    )
  }
}

export default App
