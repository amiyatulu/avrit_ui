import React, { useContext } from "react"
import { Link, useLocation } from "react-router-dom"
import { NearContext } from "../../commons/context/NearContext"

function CreateProfileLink() {
  let location = useLocation()
  let { userIdEmpty } = useContext(NearContext)
  console.log(userIdEmpty, "userIdCreate")
  if (!userIdEmpty) {
    return <React.Fragment></React.Fragment>
  } if (location.pathname === "/createprofile") {
    return <React.Fragment></React.Fragment>
  } else if (location.pathname === "/updateprofile") {
    return <React.Fragment></React.Fragment>
  }
  return (
    <React.Fragment>
      <div className="container text-center">
        Create profile to stake or post <br />
        <Link type="button" className="btn btn-primary" to="createprofile">
          Create Profile
        </Link>
      </div>
      <div>
        <br />
      </div>
    </React.Fragment>
  )
}

export default CreateProfileLink
