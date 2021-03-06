import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import { useParams, Link } from "react-router-dom"

function GetJuryStake(props) {
  const { nearvar } = useContext(NearContext)
  const { rid, userId } = props
  const [stake, setStake] = useState(null)
  const [stakeError, setStakeError] = useState(null)

  useEffect(() => {
    async function fetchStake() {
      // console.log("rid", rid, "userId", userId)
      try {
        let data = await nearvar.contract.get_juror_stakes_js({
          review_id: parseInt(rid),
          juror_user_id: parseInt(userId),
        })
        console.log(data)
        setStake(data)
      } catch (e) {
        console.error(e)
        setStakeError(e)
      }
    }
    fetchStake()
  }, [nearvar, rid, userId])

  return (
    <React.Fragment>
      {stake ? (
        <React.Fragment>
          <p className="badge badge-secondary mr-3">Your jury Stake: {stake}</p>      
        </React.Fragment>
      ) : stakeError ? (
        <React.Fragment>
          <Link
            to={`/applyjury/${rid}/`}
            className="badge badge-secondary mr-3"
          >
            Apply as Jury
          </Link>
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

export default GetJuryStake
