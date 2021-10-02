import React, { useState, useContext, useEffect } from "react"
import { useScrollHandler } from "./hooks/ScrollHandle"
// import Logo from "./image/logobg.png"
import { Link, NavLink } from "react-router-dom"
import { NearContext } from "../commons/context/NearContext"
import AvritToken from "./profile/AvritToken"
import "./Nav.css"

function Nav(props) {
  const scroll = useScrollHandler()
  let { nearvar } = useContext(NearContext)
  return (
    <React.Fragment>
      <div className="alert alert-info text-center" role="alert">
        <button type="button" className="close" data-dismiss="alert">
          ×
        </button>
        <strong>The app is on mainnet! Live!</strong>
      </div>
      <nav
        className={
          scroll
            ? "navbar navbar-expand-lg navbar-default navbar-static-top py-3 bg-warning mainbrand"
            : "navbar navbar-expand-lg navbar-default navbar-static-top py-3 navbar-scrolled"
        }
        id="mainNav"
      >
        <div className="container">
          <a className="navbar-brand js-scroll-trigger" href="/">
            {/* <img src={Logo} width="50" height="50" class="d-inline-block align-top" alt="logo"/>  */}
            Avrit
          </a>
          <button
            className="navbar-toggler navbar-toggler-right"
            type="button"
            data-toggle="collapse"
            data-target="#navbarResponsive"
            aria-controls="navbarResponsive"
            aria-expanded="false"
            aria-label="Toggle navigation"
          >
            <span className="navbar-toggler-icon"></span>
          </button>
          <div className="collapse navbar-collapse" id="navbarResponsive">
            <ul className="navbar-nav ml-auto my-2 my-lg-0">
              {props.login ? (
                <React.Fragment>
                  <li className="nav-item">
                    <Link className="nav-link js-scroll-trigger" to="/products">
                      Products
                    </Link>
                  </li>
                  <li className="dropdown">
                    <Link
                      className="dropdown-toggle nav-link"
                      data-toggle="dropdown"
                      role="button"
                      aria-haspopup="true"
                      aria-expanded="false"
                      to="/myproducts"
                    >
                      Profile
                      <span className="caret"></span>
                    </Link>
                    <ul className="dropdown-menu submenu">
                      <li>
                        <Link
                          className="nav-link js-scroll-trigger"
                          to="/profile"
                        >
                          My Profile
                        </Link>
                      </li>
                      <li>
                        <Link
                          className="nav-link js-scroll-trigger"
                          to="/myproducts"
                        >
                          My Products
                        </Link>
                      </li>

                      <li>
                        <Link
                          className="nav-link js-scroll-trigger"
                          to="/viewnft"
                        >
                          My NFTs
                        </Link>
                      </li>
                    </ul>
                  </li>
                  <li className="nav-item">
                    <Link className="nav-link js-scroll-trigger" to="/content">
                      Review Guidelines
                    </Link>
                  </li>
                  <li className="nav-item">
                    <Link className="nav-link js-scroll-trigger" to="/process">
                      Walkthrough
                    </Link>
                  </li>
                  <li className="nav-item">
                    <Link
                      className="nav-link js-scroll-trigger"
                      to="/crowdsale"
                    >
                      Crowdsale
                    </Link>
                  </li>
                  {/* <li className="nav-item">
                    <Link
                      className="nav-link js-scroll-trigger"
                      to="/createproducttopics"
                    >
                      Create Products
                    </Link>
                  </li> */}
                  <li className="nav-item">
                    <button
                      className="btn nav-link js-scroll-trigger"
                      onClick={props.onClick}
                    >
                      {nearvar.wallet.getAccountId()} (Log out)
                    </button>
                  </li>
                  <li className="nav-item">
                    <span className="btn nav-link js-scroll-trigger">
                      <AvritToken />
                    </span>
                  </li>
                </React.Fragment>
              ) : (
                <React.Fragment>
                  <li className="nav-item">
                    <Link className="nav-link js-scroll-trigger" to="/products">
                      Products
                    </Link>
                  </li>
                  <li className="nav-item">
                    <Link className="nav-link js-scroll-trigger" to="/content">
                      Review Guidelines
                    </Link>
                  </li>
                  <li className="nav-item">
                    <Link className="nav-link js-scroll-trigger" to="/process">
                      Walkthrough
                    </Link>
                  </li>
                  <li className="nav-item">
                    <Link
                      className="nav-link js-scroll-trigger"
                      to="/crowdsale"
                    >
                      Crowdsale
                    </Link>
                  </li>
                  <li className="nav-item">
                    <button
                      className="btn nav-link js-scroll-trigger"
                      onClick={props.onClick}
                    >
                      Log in
                    </button>
                  </li>
                </React.Fragment>
              )}
            </ul>
          </div>
        </div>
      </nav>
    </React.Fragment>
  )
}

export default Nav
