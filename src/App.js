import React, { useState, useEffect, useCallback, Suspense, lazy } from "react"
import { Route, Switch } from "react-router-dom"
import { NearContext } from "./commons/context/NearContext"

const CreateProfile = lazy(() => import("./components/profile/CreateProfile"))
const Nav = lazy(() => import("./components/Nav"))
const ViewProfile = lazy(() => import("./components/profile/ViewProfile"))
const UpdateProfile = lazy(() => import("./components/profile/UpdateProfile"))
const CreateProduct = lazy(() => import("./components/products/CreateProduct"))
// const GetProducts = lazy(() => import("./components/products/GetProducts"))
const ProductById = lazy(() => import("./components/products/ProductById"))
const AvritToken = lazy(() => import("./components/profile/AvritToken"))
const CreateReviewEvidence = lazy(() =>
  import("./components/reviews/CreateReviewEvidence")
)
const CreateReviewStake = lazy(() =>
  import("./components/stakes/CreateReviewStake")
)
const GetReviewStake = lazy(() => import("./components/stakes/GetReviewStake"))
const ApplyJuryStake = lazy(() =>
  import("./components/schelling/ApplyJuryStake")
)
const GetJuryStake = lazy(() => import("./components/schelling/GetJuryStake"))
const CommitVote = lazy(() => import("./components/schelling/CommitVote"))
const CommitSubmitted = lazy(() =>
  import("./components/schelling/CommitSubmitted")
)
const TimeConditionRender = lazy(() =>
  import("./components/schelling/TimeConditionRender")
)
const DropProductImage = lazy(() =>
  import("./components/products/DropProductImage")
)
const DropProductPDFs = lazy(() =>
  import("./components/products/DropProductPDFs")
)
const CreateProductTopics = lazy(() =>
  import("./components/products/CreateProductTopics")
)
const CreateReview = lazy(() => import("./components/reviews/CreateReview"))
const UpdateProduct = lazy(() => import("./components/products/UpdateProduct"))
const UpdateReview = lazy(() => import("./components/reviews/UpdateReview"))
const ProductPagination = lazy(() =>
  import("./components/products/ProductPagination")
)
const DrawJuror = lazy(() => import("./components/schelling/DrawJuror"))
const UnstakeVote = lazy(() => import("./components/schelling/UnstakeVote"))
const RevealVote = lazy(() => import("./components/schelling/RevealVote"))
const DrawIncentives = lazy(() =>
  import("./components/schelling/DrawIncentives")
)

const DrawReviewerIncentives = lazy(() =>
  import("./components/schelling/DrawReviewerIncentives")
)
const DrawProductIncentives = lazy(() =>
  import("./components/schelling/DrawProductIncentives")
)

const CreateProductStake = lazy(() =>
  import("./components/stakes/CreateProductStake")
)

const Home = lazy(() => import("./components/Home"))

const ProductCrowdfunding = lazy(() =>
  import("./components/products/ProductCrowdfunding")
)
const FetchProductsPage = lazy(() =>
  import("./components/products/FetchProductsPage")
)

const IcoFormPage = lazy(() => import("./components/ico/IcoFormPage"))
const ContentBestPractices = lazy(() =>
  import("./components/pagestext/ContentBestPractices")
)
const ProductsAll = lazy(() => import("./components/products/ProductsAll"))
const Process = lazy(() => import("./components/pagestext/Process"))

const PrivacyPolicy = lazy(() => import("./components/pagestext/PrivacyPolicy"))
const SetNFTPrice = lazy(() => import("./components/NFT/SetNFTPrice"))
const BuyNFT = lazy(() => import("./components/NFT/BuyNFT"))
const ViewNFT = lazy(() => import("./components/NFT/ViewNFT"))
const NFTBalance = lazy(() => import("./components/NFT/NFTBalance"))
const BuyNFT2Test = lazy(() => import("./components/NFT/BuyNFT2Test"))

const ProductData = lazy(() =>
  import("./components/schelling_product/ProductData")
)
const PApplyJuryStake = lazy(() =>
  import("./components/schelling_product/PApplyJuryStake")
)
const PDrawJuror = lazy(() =>
  import("./components/schelling_product/PDrawJuror")
)
const PCommitVote = lazy(() =>
  import("./components/schelling_product/PCommitVote")
)
const PUnstakeVote = lazy(() =>
  import("./components/schelling_product/PUnstakeVote")
)
const PRevealVote = lazy(() =>
  import("./components/schelling_product/PRevealVote")
)
const PDrawIncentives = lazy(() =>
  import("./components/schelling_product/PDrawIncentives")
)
const PDrawProductIncentives = lazy(() =>
  import("./components/schelling_product/PDrawProductIncentives")
)

const StakeDisapproval = lazy(() =>
  import("./components/schelling_product/StakeDisapproval")
)
const PDisapprovalProductIncentives = lazy(() =>
  import("./components/schelling_product/PDisapprovalProductIncentives")
)

function App(props) {
  const [login, setLogin] = useState(false)
  const [speech, setSpeech] = useState(null)
  const [balance, setBalance] = useState(null)
  const [balanceError, setBalanceError] = useState(null)
  const [userId, setUserId] = useState(null)
  const [userIdEmpty, setUserIdEmpty] = useState(false)

  async function fetchProfile() {
    let data
    try {
      data = await props.contract.ft_balance_of({
        account_id: props.wallet.getAccountId(),
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
      console.log("userid", userid)
      // console.log("fetchuserid")
    } catch (e) {
      console.log(e.message)
      const failedtofetch = e.message
      setUserIdEmpty(true)
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
      value={{
        nearvar: props,
        reloadBalance,
        balance,
        balanceError,
        userId,
        login,
        userIdEmpty,
        callUserId,
        setUserIdEmpty,
        requestSignIn,
      }}
    >
      <Suspense
        fallback={
          <React.Fragment>
            <div className="container">
              <div className="d-flex justify-content-center">
                <div className="spinner-grow text-warning" role="status">
                  <span className="sr-only">Loading...</span>
                </div>
              </div>
            </div>
          </React.Fragment>
        }
      >
        <React.Fragment>
          {login ? (
            <Nav onClick={requestSignOut} login={login} />
          ) : (
            <Nav onClick={requestSignIn} login={login} />
          )}
          <Switch>
            <Route path="/" exact component={Home} />
          </Switch>
          <section className="page-section">
            <Switch>
              <Route path="/createprofile" component={CreateProfile} />
              <Route path="/profile" component={ViewProfile} />
              <Route path="/updateprofile" component={UpdateProfile} />
              <Route path="/createproductold" component={CreateProduct} />
              {/* <Route path="/myproducts" component={GetProducts} /> */}
              <Route path="/product/:id" component={ProductById} />
              <Route path="/balance" component={AvritToken} />
              <Route
                path="/createreviewold/:pid"
                component={CreateReviewEvidence}
              />
              <Route path="/reviewstake/:rid" component={CreateReviewStake} />
              {/* <Route path="/getreviewstake/:rid" component={GetReviewStake} /> */}
              <Route path="/applyjury/:rid" component={ApplyJuryStake} />
              <Route
                path="/getjurystake/:rid/:userId"
                component={GetJuryStake}
              />
              {/* <Route path="/juryapplytime/:rid" component={JuryApplyTime} /> Remove it later*/}
              <Route path="/commitvote/:rid" component={CommitVote} />
              <Route path="/commitsubmitted" component={CommitSubmitted} />
              <Route
                path="/timecondition/:rid"
                component={TimeConditionRender}
              />
              {/* <Route path="/uploadimage" component={DropProductImage} /> */}
              {/* <Route path="/uploadpdf" component={DropProductPDFs} /> */}
              <Route
                path="/createproducttopics"
                component={CreateProductTopics}
              />
              <Route path="/createreview/:pid" component={CreateReview} />
              <Route path="/createproduct/:pt" component={CreateProduct} />
              <Route path="/updateproduct/:pid" component={UpdateProduct} />
              <Route path="/updatereview/:rid" component={UpdateReview} />
              {/* <Route path="/tag" component={TagsStyle} /> */}
              <Route path="/myproducts" component={ProductPagination} />
              <Route path="/drawjurors/:rid" component={DrawJuror} />
              <Route path="/unstake/:rid" component={UnstakeVote} />
              <Route path="/revealvote/:rid" component={RevealVote} />
              <Route
                path="/drawjurorincentives/:rid"
                component={DrawIncentives}
              />
              <Route
                path="/drawreviewerincentives/:rid"
                component={DrawReviewerIncentives}
              />
              <Route
                path="/drawproductincentives/:pid/:rid"
                component={DrawProductIncentives}
              />
              <Route path="/productstake/:id" component={CreateProductStake} />
              <Route
                path="/productcrowdfund/:pid"
                component={ProductCrowdfunding}
              />
              <Route path="/fetchproducts" component={FetchProductsPage} />
              <Route path="/crowdsale" component={IcoFormPage} />
              <Route path="/content" component={ContentBestPractices} />
              <Route path="/products" component={ProductsAll} />
              <Route path="/process" component={Process} />
              <Route path="/privacypolicy" component={PrivacyPolicy} />
              <Route path="/setnftprice/:pid" component={SetNFTPrice} />
              <Route path="/buynft/:pid" component={BuyNFT} />
              <Route path="/buynft2/:pid" component={BuyNFT2Test} />
              <Route path="/viewnft" component={ViewNFT} />
              <Route path="/nftbalance" component={NFTBalance} />
              <Route path="/productdata/:pid" component={ProductData} />
              <Route
                path="/productapplyjury/:pid"
                component={PApplyJuryStake}
              />
              <Route path="/drawjuryforproduct/:pid" component={PDrawJuror} />
              <Route path="/commitvoteproduct/:pid" component={PCommitVote} />
              <Route path="/unstakeproduct/:pid" component={PUnstakeVote} />
              <Route path="/revealvoteproduct/:pid" component={PRevealVote} />
              <Route
                path="/drawincentivesjuryproduct/:pid"
                component={PDrawIncentives}
              />
              <Route
                path="/productdrawincentives/:pid"
                component={PDrawProductIncentives}
              />
              <Route
                path="/stakedisapproval/:id"
                component={StakeDisapproval}
              />
              <Route
                path="/drawdisapprovalincentives/:pid"
                component={PDisapprovalProductIncentives}
              />
            </Switch>
          </section>
        </React.Fragment>
      </Suspense>
    </NearContext.Provider>
  )
}

export default App
