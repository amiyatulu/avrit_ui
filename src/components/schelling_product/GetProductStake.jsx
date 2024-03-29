import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import { useParams, Link } from "react-router-dom"
import axios from "axios"
import PGetJuryStake from "./PGetJuryStake"
import { BigNumber } from "bignumber.js";
import PTimeConditionRender from "./PTimeConditionRender"
import loading from "../image/EllipsisLoading.gif"
import StakeDisapprovalLink from "./StakeDisapprovalLink"
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
      <span role="img" aria-label="loading">
        <img src={loading} alt="loading" />
        
      </span>
    </p>
  )
}
function GetProductStake(props) {
  const { nearvar, userId } = useContext(NearContext)
  const { pid } = props
  const [stake, setStake] = useState(null)
  const [noStake, setNoStake] = useState(null)
  let pw = BigNumber(10).pow(18)
  useEffect(() => {
    async function fetchStake() {
      try {
        let data = await nearvar.contract.get_product_bounty_js({
          product_id: pid.toString(),
        })
        console.log(data)
        setStake(data)
      } catch (e) {
        console.error(e)
        setNoStake(e.message)
      }
    }
    fetchStake()
  }, [nearvar, pid])
  return (
    <React.Fragment>
      {stake ? (
        <React.Fragment>
        <p className="badge badge-secondary mr-3">Product Stake: {BigNumber(stake).div(pw).toFixed()} </p>
       
        <StakeDisapprovalLink pid={pid} useId={userId} />
        <PTimeConditionRender pid={pid}/>
        </React.Fragment>
      ) : (
        <LoadingOrNoStake noStake={noStake} />
      )}
    </React.Fragment>
  )
}

export default GetProductStake
