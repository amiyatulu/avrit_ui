import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import axios from "axios"
import { Link, NavLink } from "react-router-dom"
import { IPFS_URL } from "../../config/configvar"
import styles from "./ViewProfile.module.css"
import { BigNumber } from "bignumber.js";

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
  let pw = BigNumber(10).pow(18)

  console.log("balance", balance)
  return (
    <React.Fragment>
      {balance ? (
        <React.Fragment>
          <span>Balance: {BigNumber(balance).div(pw).toFixed()}</span>
        </React.Fragment>
      ) : (
        <Loading fetchError={balanceError} />
      )}
    </React.Fragment>
  )
}

export default AvritToken
