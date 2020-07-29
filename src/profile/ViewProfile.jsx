import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../context/NearContext"
import axios from "axios"
import { Link, NavLink } from "react-router-dom"
import { IPFS_URL } from "../config/configvar"
import styles from "./ViewProfile.module.css"

function LoadingOrCreateProfile(props) {
  const { noProfile, fetchError } = props
  if (fetchError) {
    return <p className="container">{fetchError}</p>
  }
  if (noProfile) {
    return (
      <React.Fragment>
        <div className="text-center">
          <Link type="button" className="btn btn-primary" to="createprofile" >
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
  const [fetchError, setFetchError] = useState(false)
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
        const failedtofetch = e.message
        console.log(errorboolean)
        setNoProfile(errorboolean)
        setFetchError(failedtofetch)
      }
      if (data) {
        const result = await axios(`${IPFS_URL}${data}`)
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

  return (
    <React.Fragment>
      {profileData ? (
        <div className="container">
          <Link type="button" className="btn btn-primary" to={{ pathname:"/updateprofile", query: {profileData: profileData}}}>
            Update Profile
          </Link>
          <br />
          <br />
          <h3 className={styles.labelstyle}>Headline</h3>
          <p className={styles.profilepara}>{profileData.headline}</p>
          <h3 className={styles.labelstyle}>Introduction</h3>
          <p className={styles.profilepara}>{profileData.introduction}</p>
          <h3 className={styles.labelstyle}>Details</h3>
          <p className={styles.profilepara}>{profileData.details}</p>
          <h3 className={styles.labelstyle}>You are</h3>
          <p className={styles.profilepara}>{profileData.youAre}</p>
          <h3 className={styles.labelstyle}>Skills</h3>
          <p className={styles.profilepara}>{profileData.skills}</p>
          {/* <pre>{JSON.stringify(profileData)}</pre> */}
        </div>
      ) : (
        <LoadingOrCreateProfile noProfile={noProfile} fetchError={fetchError} />
      )}
    </React.Fragment>
  )
}

export default ViewProfile
