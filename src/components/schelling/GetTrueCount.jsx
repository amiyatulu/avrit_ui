import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"

function Error(props) {
  const { fetchError } = props
  if (fetchError) {
    return <span className="container">{fetchError}</span>
  }
  return <React.Fragment></React.Fragment>
}

function GetTrueCount(props) {
  const { nearvar, userId } = useContext(NearContext)
  const { rid } = props
  const [fetchError, setFetchError] = useState(false)
  const [count, setCount] = useState(0)
  useEffect(() => {
    async function getTrueCount() {
      try {
        const countvalue = await nearvar.contract.get_true_count_js({
          review_id: rid.toString(),
        })
        setCount(countvalue)
      } catch (e) {
        console.error(e.message)
        setFetchError(e.message)
      }
    }
    getTrueCount()
  }, [nearvar, userId, rid])

  return (
    <React.Fragment>
      <p className="badge badge-warning mr-3">
        Upvotes: {count}
      </p>
      <Error fetchError={fetchError} />
    </React.Fragment>
  )
}

export default GetTrueCount
