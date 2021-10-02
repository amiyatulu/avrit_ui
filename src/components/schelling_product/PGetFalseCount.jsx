import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"

function Error(props) {
  const { fetchError } = props
  if (fetchError) {
    return <span className="container">{fetchError}</span>
  }
  return <React.Fragment></React.Fragment>
}

function PGetFalseCount(props) {
  const { nearvar, userId } = useContext(NearContext)
  const { pid } = props
  const [fetchError, setFetchError] = useState(false)
  const [count, setCount] = useState(0)
  useEffect(() => {
    async function getFalseCount() {
      try {
        const countvalue = await nearvar.contract.p_get_false_count_js({
          product_id: pid.toString(),
        })
        setCount(countvalue)
      } catch (e) {
        console.error(e.message)
        setFetchError(e.message)
      }
    }
    getFalseCount()
  }, [nearvar, userId, pid])

  return (
    <React.Fragment>
      <p className="badge badge-warning mr-3">
        Downvotes: {count}
      </p>
      <Error fetchError={fetchError} />
    </React.Fragment>
  )
}

export default PGetFalseCount
