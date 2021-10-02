import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"

function Error(props) {
  const { fetchError } = props
  if (fetchError) {
    return <span className="container">{fetchError}</span>
  }
  return <React.Fragment></React.Fragment>
}

function PJuryCount(props) {
  const { nearvar, userId } = useContext(NearContext)
  const { pid } = props
  const [fetchError, setFetchError] = useState(false)
  const [count, setCount] = useState(0)
  useEffect(() => {
    async function fetchjurycount() {
      try {
        const countvalue = await nearvar.contract.p_number_of_staked_jury({
          product_id: pid.toString(),
        })
        setCount(countvalue)
      } catch (e) {
        console.error(e.message)
        setFetchError(e.message)
      }
    }
    fetchjurycount()
  }, [nearvar, userId, pid])

  return (
    <React.Fragment>
      <p className="badge badge-secondary mr-3">
        Number of Jury applied: {count}
      </p>
      <Error fetchError={fetchError} />
    </React.Fragment>
  )
}

export default PJuryCount
