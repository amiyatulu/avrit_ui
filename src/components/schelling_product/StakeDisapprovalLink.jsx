import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import { Link } from "react-router-dom"
import PGetJuryStake from "./PGetJuryStake"
import loading from "../image/EllipsisLoading.gif"

function Error(props) {
  const { fetchError } = props
  if (fetchError) {
    return <span className="container">{fetchError}</span>
  }
  return <React.Fragment></React.Fragment>
}

function StakeDisapprovalLink(props) {
  const { nearvar, userId } = useContext(NearContext)
  const { pid } = props
  const [fetchError, setFetchError] = useState(false)
  const [canVote, setCanVote] = useState(null)

  useEffect(() => {
    async function fetchjurycount() {
      try {
        const canVotevalue =
          await nearvar.contract.disapproval_product_bounty_display_bool({
            product_id: pid.toString(),
          })
        console.log("canVoteValue", canVotevalue)
        setCanVote(canVotevalue)
      } catch (e) {
        console.error(e.message)
        setFetchError(e.message)
      }
    }
    fetchjurycount()
  }, [nearvar, pid])
  return (
    <React.Fragment>
      {canVote && (
        <Link
          to={`/stakedisapproval/${pid}/`}
          className="badge badge-secondary mr-3"
        >
          Disapproval Stake
        </Link>
      )}

      {canVote === false && <PGetJuryStake pid={pid} userId={userId} />}

      {/* <p> Can You Commit Vote {JSON.stringify(canVote)}</p> */}
      {userId && <Error fetchError={fetchError} />}
      {canVote === null && !fetchError && (
        <span role="img" aria-label="loading">
          <img src={loading} alt="loading" />
        </span>
      )}
    </React.Fragment>
  )
}

export default StakeDisapprovalLink
