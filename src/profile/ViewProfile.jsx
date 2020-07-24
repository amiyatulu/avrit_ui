import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../context/NearContext"
import axios from "axios"
import { Link, NavLink } from "react-router-dom"

function LoadingOrCreateProfile(props) {
  const noProfile = props.noProfile
  if (noProfile) {
    return (
      <React.Fragment>
        <div className="text-center">
        <Link type="button" className="btn btn-primary" to="createprofile">
          Create Profile
        </Link>
        </div>
      </React.Fragment>
    )
  }
  return (
    <p className="container">
      Loading
      <span role="img" aria-label="loading">
        ⌛⌛⌛⌛
      </span>
    </p>
  )
}

function ViewProfile() {
  const [profileData, setProfileData] = useState(null)
  const [noProfile, setNoProfile] = useState(false)
  const nearcontract = useContext(NearContext)



  useEffect(() => {
    async function fetchProfile() {
      let data
      try {
        data = await nearcontract.contract.get_profile_hash()
      } catch (e) {
        console.log(e.message)
        const errorboolean = e.message.includes("User profile does not exists")
        console.log(errorboolean)
        setNoProfile(errorboolean)
      }
      if (data) {
        const result = await axios(`https://gateway.ipfs.io/ipfs/${data}`)
        setProfileData(result.data)
        localStorage.setItem("my-profile", JSON.stringify(result.data))
        console.log("in useeffect")
      }
    }
    const profileLocalData = localStorage.getItem("my-profile")
    if (profileLocalData) {
      setProfileData(JSON.parse(profileLocalData))
    } else {
      fetchProfile()
    }
  }, [nearcontract])

  useEffect(() => {
    const removeProfile = () => {
      localStorage.removeItem("my-profile")
    }
    window.addEventListener("beforeunload", removeProfile);

    return () => window.removeEventListener("beforeunload", removeProfile);

  }, [])

  return (
    <React.Fragment>
      {profileData ? (
        <div className="container">
          <Link type="button" className="btn btn-primary" to="createprofile">
            Update Profile
          </Link>
          <br />
          <br />
          <pre>{JSON.stringify(profileData)}</pre>
        </div>
      ) : (
        <LoadingOrCreateProfile noProfile={noProfile} />
      )}
    </React.Fragment>
  )
}

export default ViewProfile
