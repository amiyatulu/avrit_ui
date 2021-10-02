import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import { BigNumber } from "bignumber.js"

function GetReviewMinStake(props) {
  const { nearvar } = useContext(NearContext)
  const { rid, userId } = props
  const [stake, setStake] = useState(null)
  //   const [stakeError, setStakeError] = useState(null)
  let pw = BigNumber(10).pow(18)

  useEffect(() => {
    async function fetchStake() {
      // console.log("rid", rid, "userId", userId)
      try {
        let data = await nearvar.contract.get_min_review_bounty({})
        console.log(data)
        setStake(data)
      } catch (e) {
        console.error(e)
      }
    }
    fetchStake()
  }, [nearvar, rid, userId])

  return (
    <React.Fragment>
      <p className="badge badge-secondary mr-3">
        Your min review stake: {BigNumber(stake).div(pw).toFixed()}
      </p> 
    </React.Fragment>
  )
}

export default GetReviewMinStake
