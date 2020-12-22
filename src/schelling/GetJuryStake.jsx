import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../context/NearContext"
import { useParams, Link } from "react-router-dom"

function GetJuryStake(props) {
  const { nearvar } = useContext(NearContext)
  const { rid, userId } = props
  const [stake, setStake] = useState(null)

  useEffect(() => {
    async function fetchStake() {
      // console.log("rid", rid, "userId", userId)
      try {
        // let userId =
        let data = await nearvar.contract.get_juror_stakes_js({
          review_id: parseInt(rid),
          juror_user_id: parseInt(userId),
        })
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
      <p className="badge badge-secondary mr-3">Your Stake: {stake} </p>
    </React.Fragment>
  )
}

export default GetJuryStake
