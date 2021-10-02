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
   
   
function PUnstakeVoteLink(props) {
    const { nearvar, userId } = useContext(NearContext)
    const { pid } = props
    const [fetchError, setFetchError] = useState(false)
    const [canUnstake, setCanUnstake] = useState(false)

    useEffect(() => {
        async function fetchjurycount() {
          try {
            const canUnstakevalue = await nearvar.contract.p_can_juror_unstake_bool({
              product_id: pid.toString(),
              user_id: userId.toString(),
            })
            console.log("canUnstakeValue", canUnstakevalue)
            setCanUnstake(canUnstakevalue)
          } catch (e) {
            console.error(e.message)
            setFetchError(e.message)
          }
        }
        fetchjurycount()
      }, [nearvar, userId, pid])
      return (
          <React.Fragment>
            {canUnstake && <span className="badge badge-info">You are not selected as juror</span>} <br/>
            { canUnstake && <Link to={`/unstakeproduct/${pid}/`} className="badge badge-secondary mr-3">
         Unstake
        </Link>}
           {/* <p> Can You Commit Vote {JSON.stringify(canUnstake)}</p> */}
           {userId && <Error fetchError={fetchError} />}
           </React.Fragment>
       );
}
  
  
export default PUnstakeVoteLink