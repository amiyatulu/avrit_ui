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
   
   
function PCommitVoteLink(props) {
    const { nearvar, userId } = useContext(NearContext)
    const { pid } = props
    const [fetchError, setFetchError] = useState(false)
    const [canVote, setCanVote] = useState(false)

    useEffect(() => {
        async function fetchjurycount() {
          try {
            const canVotevalue = await nearvar.contract.p_can_juror_vote_bool({
              product_id: pid.toString(),
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
      }, [nearvar, userId, pid])
      return (
          <React.Fragment>
            { canVote && <Link to={`/commitvoteproduct/${pid}/`} className="badge badge-secondary mr-3">
          Commit Vote
        </Link>}
           {/* <p> Can You Commit Vote {JSON.stringify(canVote)}</p> */}
           {userId && <Error fetchError={fetchError} />}
           </React.Fragment>
       );
}
  
  
export default PCommitVoteLink