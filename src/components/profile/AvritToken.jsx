import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import axios from "axios"
import { Link, NavLink } from "react-router-dom"
import { IPFS_URL } from "../../config/configvar"
import styles from "./ViewProfile.module.css"
// import BN from "bn.js"

function Loading(props) {
  const { fetchError } = props
  if (fetchError) {
    return <p className="container">{fetchError}</p>
  }

  return (
    <p className="container">
      <span role="img" aria-label="loading">
        Balance: ⌛
      </span>
    </p>
  )
}

function AvritToken() {
  const { balance, balanceError } = useContext(NearContext)
  // let balancebn = new BN(balance)
  // let power = new BN("1000000000000000000")
  // let balancenow = balancebn.div(power)
  return (
    <React.Fragment>
      {balance ? (
        <React.Fragment>
          <span>Balance: {balance  * Math.pow(10, -18)}</span>
        </React.Fragment>
      ) : (
        <Loading fetchError={balanceError} />
      )}
    </React.Fragment>
  )
}

export default AvritToken
