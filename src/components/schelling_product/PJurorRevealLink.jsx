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
   
   
function PJurorRevealLink(props) {
    const { nearvar, userId } = useContext(NearContext)
    const { pid } = props
    const [fetchError, setFetchError] = useState(false)
    const [canReveal, setCanReveal] = useState(false)

    useEffect(() => {
        async function fetchjurycount() {
          try {
            const canRevealvalue = await nearvar.contract.p_can_juror_reveal({
              product_id: pid.toString(),
              user_id: userId.toString(),
            })
            console.log("canRevealValue", canRevealvalue)
            setCanReveal(canRevealvalue)
          } catch (e) {
            console.error(e.message)
            setFetchError(e.message)
          }
        }
        fetchjurycount()
      }, [nearvar, userId, pid])
      return (
          <React.Fragment>
            { canReveal && <Link to={`/revealvoteproduct/${pid}/`} className="badge badge-secondary mr-3">
          Reveal Vote
        </Link>}
           {/* <p> Can You Commit Vote {JSON.stringify(canReveal)}</p> */}
           {userId && <Error fetchError={fetchError} />}
           </React.Fragment>
       );
}
  
  
export default PJurorRevealLink