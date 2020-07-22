import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../context/NearContext"
import axios from "axios"
import { Link, NavLink } from "react-router-dom"

function ViewProfile() {
  const [profileData, setProfileData] = useState(null)
  const nearcontract = useContext(NearContext)

  useEffect(() => {
    async function fetchProfile() {
      const data = await nearcontract.contract.get_profile_hash()
      const result = await axios(`https://gateway.ipfs.io/ipfs/${data}`)
      setProfileData(result.data)
      localStorage.setItem("my-profile", JSON.stringify(result.data))
      console.log("in useeffect")
    }
    const data = localStorage.getItem("my-profile")
    if (data) {
      setProfileData(JSON.parse(data))
    } else {
      fetchProfile()
    }
  }, [nearcontract])

  return (
    <React.Fragment>
      {profileData ? (
        <div className="container">
          <Link type="button" className="btn btn-primary" to="updateprofile">
            Update Profile
          </Link>
          <br/><br/>
          <pre>{JSON.stringify(profileData)}</pre>
        </div>
      ) : (
        <p className="container">
          Loading
          <span role="img" aria-label="loading">
            ⌛⌛⌛⌛
          </span>
        </p>
      )}
    </React.Fragment>
  )
}

export default ViewProfile
