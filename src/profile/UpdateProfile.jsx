import React, { useState, useContext } from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { FocusError, SubmittingWheel } from "../commons/FocusWheel"
import { NearContext } from "../context/NearContext"
import ipfs from "../commons/ipfs"
import ProfileForm from "./ProfileForm"
import { useHistory } from "react-router-dom"
import Ipfsadd from "../commons/TextileIO"

function UpdateProfile(props) {
  let { nearvar } = useContext(NearContext)
  const defaultProfileData = {
    profileData: "",
  }
  const { profileData } = props.location.query || defaultProfileData
  console.log(profileData)
  let initialValues = {
    headline: "",
    introduction: "",
    details: "",
    youAre: "",
    skills: "",
  }
  if (profileData) {
    initialValues = {
      headline: profileData.headline,
      introduction: profileData.introduction,
      details: profileData.details,
      youAre: profileData.youAre,
      skills: profileData.skills,
    }
  }
  const history = useHistory()
  const [fetchError, setFetchError] = useState(null)

  return (
    <React.Fragment>
      <div className="container">
        <Formik
          initialValues={initialValues}
          validationSchema={Yup.object().shape({
            headline: Yup.string().required("Headline is required"),
            introduction: Yup.string().required("Introduction is required"),
            details: Yup.string().required("Details is required"),
            youAre: Yup.string().required("You are is required"),
            skills: Yup.string().required("Skills is required"),
          })}
          onSubmit={async (values, actions) => {
            //values.countvariable = count
            console.log(values)
            try {
              const file = await ipfs.add({
                path: "profile.json",
                content: JSON.stringify(values),
              })

              console.log(file)
              console.log(file.cid.string)
              console.log(nearvar.contract)
              await nearvar.contract.update_profile({
                profile_hash: file.cid.string,
              })

              // const content = JSON.stringify(values);
              // const filename = "profile.json"
              // const data = await Ipfsadd(content, filename)
              // console.log(data, "mydata")
              // await nearvar.contract.create_profile({ profile_hash: data.path.cid.string })
              localStorage.removeItem("my-profile")
              history.push("/profile")
            } catch (e) {
              console.error(e)
              setFetchError("Submission error, try again!")
            }

            // actions.setSubmitting(false)
            // console.log(data)
          }}
        >
          {({
            handleSubmit,
            handleBlur,
            handleChange,
            errors,
            touched,
            isSubmitting,
            values,
            setFieldValue,
            validateForm,
          }) => (
            <Form onSubmit={handleSubmit}>
              <ProfileForm
                errors={errors}
                touched={touched}
                isSubmitting={isSubmitting}
              />
              <br />
              <div className="text-center">{fetchError}</div>
              <SubmittingWheel isSubmitting={isSubmitting} />

              <FocusError />
            </Form>
          )}
        </Formik>
      </div>
    </React.Fragment>
  )
}

export default UpdateProfile
