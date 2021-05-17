import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"

function Error(props) {
  const { fetchError } = props
  if (fetchError) {
    return <span className="container">{fetchError}</span>
  }
  return <React.Fragment></React.Fragment>
}

function UserName(props) {
  const { nearvar, userId } = useContext(NearContext)
  const { uid } = props
  const [fetchError, setFetchError] = useState(false)
  const [username, setUsername] = useState("")
  useEffect(() => {
    async function getUsername() {
      try {
        const username = await nearvar.contract.get_username({
            user_id: uid.toString(),
        })
        setUsername(username)
      } catch (e) {
        console.error(e.message)
        setFetchError(e.message)
      }
    }
    getUsername()
  }, [nearvar, uid])

  return (
    <React.Fragment>
        <br/>
      <p className="badge badge-success mr-3 float-right">
        {username}
      </p>
      <Error fetchError={fetchError} />
    </React.Fragment>
  )
}

export default UserName
