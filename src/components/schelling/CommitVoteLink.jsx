import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import { Link } from "react-router-dom"

function Error(props) {
  const { fetchError } = props
  if (fetchError) {
    return <span className="container">{fetchError}</span>
  }
  return <React.Fragment></React.Fragment>
}
   
   
function CommitVoteLink(props) {
    const { nearvar, userId } = useContext(NearContext)
    const { rid } = props
    const [fetchError, setFetchError] = useState(false)
    const [canVote, setCanVote] = useState(false)

    useEffect(() => {
        async function fetchjurycount() {
          try {
            const canVotevalue = await nearvar.contract.can_juror_vote_bool({
              review_id: rid.toString(),
              user_id: userId.toString(),
            })
            console.log("canVoteValue", canVotevalue)
            setCanVote(canVotevalue)
          } catch (e) {
            console.error(e.message)
            setFetchError(e.message)
          }
        }
        fetchjurycount()
      }, [nearvar, userId, rid])
      return (
          <React.Fragment>
            { canVote && <Link to={`/commitvote/${rid}/`} className="badge badge-secondary mr-3">
          Commit Vote
        </Link>}
           {/* <p> Can You Commit Vote {JSON.stringify(canVote)}</p> */}
           {userId && <Error fetchError={fetchError} />}
           </React.Fragment>
       );
}
  
  
export default CommitVoteLink