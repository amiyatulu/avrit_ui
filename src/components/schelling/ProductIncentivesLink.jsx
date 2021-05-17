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
   
   
function ProductIncentivesLink(props) {
    const { nearvar, userId } = useContext(NearContext)
    const { rid, pid } = props
    const [fetchError, setFetchError] = useState(false)
    const [canDrawIncentives, setCanDrawIncentives] = useState(false)

    useEffect(() => {
        async function fetchproductincentives() {
          try {
            const canDrawIncentivesvalue = await nearvar.contract.check_product_will_get_incentives_bool({
              review_id: rid.toString(),
              product_id: pid.toString(),
            })
            console.log("canDrawIncentivesValue", canDrawIncentivesvalue)
            setCanDrawIncentives(canDrawIncentivesvalue)
          } catch (e) {
            console.error(e.message)
            setFetchError(e.message)
          }
        }
        fetchproductincentives()
      }, [nearvar, rid, pid])
      return (
          <React.Fragment>
            { canDrawIncentives && <span className="badge badge-info">Product creater can draw incentives for this review</span>
        }
        <br />
            { canDrawIncentives && <Link to={`/drawproductincentives/${pid}/${rid}`} className="badge badge-secondary mr-3">
          Draw Incentives for Product
        </Link>}

           {/* <p> Can You Commit Vote {JSON.stringify(canDrawIncentives)}</p> */}
           {userId && <Error fetchError={fetchError} />}
           </React.Fragment>
       );
}
  
  
export default ProductIncentivesLink