import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import axios from "axios"
import { Link, NavLink } from "react-router-dom"
import { IPFS_URL } from "../../config/configvar"
import styles from "./ViewProfile.module.css"
import CreateProfileLink from "../commondom/CreateProfileLink"

function LoadingOrCreateProfile(props) {
  const { noProfile, fetchError } = props
  if (noProfile) {
    return (
      <React.Fragment>
        {/* <div className="text-center">
          <Link type="button" className="btn btn-primary" to="createprofile">
            Create Profile
          </Link>
        </div> */}
      </React.Fragment>
    )
  }
  if (fetchError) {
    return <p className="container">{fetchError}</p>
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
  const { nearvar, userId } = useContext(NearContext)
  console.log(userId, "userId")

  useEffect(() => {
    async function fetchProfile() {
      if (userId) {
        setNoProfile(false)
        let data
        try {
          data = await nearvar.contract.get_user_profile_js({
            user_id: userId.toString(),
          })
          // console.log(data)
          const result = await axios(`${IPFS_URL}${data.profile_hash}`)
          setProfileData(result.data)
          localStorage.setItem("my-profile", JSON.stringify(result.data))
          console.log("in useeffect")
        } catch (e) {
          console.log(e.message)
          const errorboolean = e.message.includes(
            "User profile does not exists"
          )
          const failedtofetch = e.message
          console.log(errorboolean)
          setNoProfile(errorboolean)
          setFetchError(failedtofetch)
        }
      } else {
        console.log("Out of userid")
        setNoProfile(true)
      }
    }

    fetchProfile()
    const profileLocalData = localStorage.getItem("my-profile")
    if (profileLocalData) {
      setProfileData(JSON.parse(profileLocalData))
    } else {
      fetchProfile()
    }
  }, [nearvar, userId])

  return (
    <React.Fragment>
      <br/>
      <CreateProfileLink />
      {profileData ? (
        <div className="container">
          <Link
            type="button"
            className="btn btn-primary"
            to={{
              pathname: "/updateprofile",
              query: { profileData: profileData },
            }}
          >
            Update Profile
          </Link>
          <br />
          <br />
          <h3 className={styles.labelstyle}>Headline</h3>
          <p className={styles.profilepara}>{profileData.headline}</p>
          <h3 className={styles.labelstyle}>Introduction</h3>
          <p className={styles.profilepara}>{profileData.introduction}</p>
          {/* <pre>{JSON.stringify(profileData)}</pre> */}
        </div>
      ) : (
        <LoadingOrCreateProfile noProfile={noProfile} fetchError={fetchError} />
      )}
    </React.Fragment>
  )
}

export default ViewProfile
