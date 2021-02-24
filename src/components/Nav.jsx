import React, { useState, useContext, useEffect } from "react"
import { useScrollHandler } from "./hooks/ScrollHandle"
// import Logo from "./image/logobg.png"
import { Link, NavLink } from "react-router-dom"
import { NearContext } from "../commons/context/NearContext"
import AvritToken from "./profile/AvritToken"
import CreateProfileLink from "./commondom/CreateProfileLink"

function Nav(props) {
  const scroll = useScrollHandler()
  let { nearvar } = useContext(NearContext)
  return (
    <React.Fragment>
      <nav
        className={
          scroll
            ? "navbar navbar-expand-lg navbar-default fixed-top py-3 bg-warning mainbrand"
            : "navbar navbar-expand-lg navbar-default fixed-top py-3 navbar-scrolled"
        }
        id="mainNav"
      >
        <div className="container">
          <a className="navbar-brand js-scroll-trigger" href="#page-top">
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
              <li className="nav-item">
                <a className="nav-link js-scroll-trigger" href="#about">
                  About
                </a>
              </li>
              <li className="nav-item">
                <a
                  className="nav-link js-scroll-trigger"
                  href="paper/AvritWhitePaper.pdf"
                >
                  WHITEPAPER
                </a>
              </li>
              <li className="nav-item">
                <a
                  className="nav-link js-scroll-trigger"
                  href="https://github.com/amiyatulu/avrit_ui"
                >
                  Source Code
                </a>
              </li>

              {props.login ? (
                <React.Fragment>
                  <li className="nav-item">
                    <Link className="nav-link js-scroll-trigger" to="/profile">
                      Profile
                    </Link>
                  </li>
                  <li className="nav-item">
                    <Link
                      className="nav-link js-scroll-trigger"
                      to="/myproducts"
                    >
                      Products
                    </Link>
                  </li>
                  <li className="nav-item">
                    <button
                      className="btn nav-link js-scroll-trigger"
                      onClick={props.onClick}
                    >
                      {nearvar.wallet.getAccountId()} (Log out)
                    </button>
                  </li>
                  <li className="nav-item">
                    <button className="btn nav-link js-scroll-trigger">
                      <AvritToken />
                    </button>
                  </li>
                </React.Fragment>
              ) : (
                <li className="nav-item">
                  <button
                    className="btn nav-link js-scroll-trigger"
                    onClick={props.onClick}
                  >
                    Log in
                  </button>
                </li>
              )}
            </ul>
          </div>
        </div>
      </nav>
      <CreateProfileLink/>
    </React.Fragment>
  )
}

export default Nav
