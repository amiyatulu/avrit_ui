import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import axios from "axios"
import { Link, NavLink } from "react-router-dom"
import { IPFS_URL } from "../../config/configvar"
import styles from "./ViewProfile.module.css"

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
  const [balanceData, setBalanceData] = useState(null)
  const { balance, balanceError } = useContext(NearContext)
  return (
    <React.Fragment>
      {balance ? (
        <span>Balance: {balance}</span>
      ) : (
        <Loading fetchError={balanceError} />
      )}
    </React.Fragment>
  )
}

export default AvritToken
