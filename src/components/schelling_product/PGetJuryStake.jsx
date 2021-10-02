import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import { useParams, Link } from "react-router-dom"
import PJuryCount from "./PJuryCount"
import { BigNumber } from "bignumber.js";

function PGetJuryStake(props) {
  const { nearvar } = useContext(NearContext)
  const { pid, userId } = props
  const [stake, setStake] = useState(null)
  const [stakeError, setStakeError] = useState(null)
  let pw = BigNumber(10).pow(18)

  useEffect(() => {
    async function fetchStake() {
      // console.log("rid", rid, "userId", userId)
      try {
        let data = await nearvar.contract.p_get_juror_stakes_js({
          product_id: pid.toString(),
          juror_user_id: userId.toString(),
        })
        console.log(data)
        setStake(data)
      } catch (e) {
        console.error(e)
        setStakeError(e)
      }
    }
    fetchStake()
  }, [nearvar, pid, userId])

  return (
    <React.Fragment>
      {stake ? (
        <React.Fragment>
          <p className="badge badge-secondary mr-3">Your jury Stake: {BigNumber(stake).div(pw).toFixed()}</p>
          <PJuryCount pid={pid} />
        </React.Fragment>
      ) : stakeError ? (
        <React.Fragment>
          <Link
            to={`/productapplyjury/${pid}/`}
            className="badge badge-secondary mr-3"
          >
            Apply as Jury
          </Link>
          <PJuryCount pid={pid} />

        </React.Fragment>
      ) : (
        <React.Fragment>
          <span role="img" aria-label="loading">
            ⌛
          </span>
        </React.Fragment>
      )}
    </React.Fragment>
  )
}

export default PGetJuryStake
