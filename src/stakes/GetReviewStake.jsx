import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../context/NearContext"
import { useParams, Link } from "react-router-dom"
import axios from "axios"
function LoadingOrNoStake(props) {
  const { noStake } = props
  if (noStake) {
    return (
      <React.Fragment>
        <p className="badge badge-secondary mr-3">Stake: 0</p>
      </React.Fragment>
    )
  }
  return (
    <p className="container">
      Loading
      <span role="img" aria-label="loading">
        ⌛
      </span>
    </p>
  )
}
function GetReviewStake(props) {
  const { nearvar } = useContext(NearContext)
  const { rid } = props
  const [stake, setStake] = useState(null)
  const [noStake, setNoStake] = useState(null)
  useEffect(() => {
    async function fetchStake() {
      try {
        let data = await nearvar.contract.get_review_bounty_js({
          review_id: parseInt(rid),
        })
        console.log(data)
        setStake(data)
      } catch (e) {
        console.error(e)
        const noStakeError = e.message.includes("Bounty does not exists")
        setNoStake(noStakeError)
      }
    }
    fetchStake()
  }, [nearvar, rid])
  return (
    <React.Fragment>
      {stake ? (
        <p className="badge badge-secondary mr-3">Stake: {stake[0]} </p>
      ) : (
        <LoadingOrNoStake noStake={noStake} />
      )}
    </React.Fragment>
  )
}

export default GetReviewStake
