import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import { BigNumber } from "bignumber.js"

function PGetMinJuryStake(props) {
  const { nearvar } = useContext(NearContext)
  const { pid, userId } = props
  const [stake, setStake] = useState(null)
  //   const [stakeError, setStakeError] = useState(null)
  let pw = BigNumber(10).pow(18)

  useEffect(() => {
    async function fetchStake() {
      // console.log("rid", rid, "userId", userId)
      try {
        let data = await nearvar.contract.p_get_min_jury_stake({})
        console.log(data)
        setStake(data)
      } catch (e) {
        console.error(e)
      }
    }
    fetchStake()
  }, [nearvar, pid, userId])

  return (
    <React.Fragment>
      <p className="badge badge-secondary mr-3">
        Your min jury stake: {BigNumber(stake).div(pw).toFixed()}
      </p> <br/>
      <p className="badge badge-secondary mr-3">

        Higher the stake, more the chance to be selected as juror
      </p>
    </React.Fragment>
  )
}

export default PGetMinJuryStake
