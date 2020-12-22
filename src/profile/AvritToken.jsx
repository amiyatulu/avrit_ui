import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../context/NearContext"
import axios from "axios"
import { Link, NavLink } from "react-router-dom"
import { IPFS_URL } from "../config/configvar"
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
  const [fetchError, setFetchError] = useState(false)
  const { nearvar } = useContext(NearContext)

  useEffect(() => {
    async function fetchProfile() {
      let data
      try {
        data = await nearvar.contract.get_balance({
          owner_id: nearvar.wallet.getAccountId(),
        })
        console.log(nearvar.wallet.getAccountId())
        console.log(data)
        setBalanceData(data)
      } catch (e) {
        console.log(e.message)
        const failedtofetch = e.message
        setFetchError(failedtofetch)
      }
    }

    fetchProfile()
  }, [nearvar])

  return (
    <React.Fragment>
      {balanceData ? (
        <span>Balance: {balanceData}</span>
      ) : (
        <Loading fetchError={fetchError} />
      )}
    </React.Fragment>
  )
}

export default AvritToken
