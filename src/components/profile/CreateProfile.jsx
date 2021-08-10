import React, { useState, useContext } from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { FocusError, SubmittingWheel } from "../../commons/FocusWheel"
import { NearContext } from "../../commons/context/NearContext"
import ipfs from "../../commons/ipfs"
import ProfileForm from "./ProfileForm"
import { useHistory } from "react-router-dom"
import Ipfsadd from "../../commons/TextileIO"

function CreateProfile() {
  const [count, setCount] = useState(0)
  let { nearvar, callUserId, setUserIdEmpty } = useContext(NearContext)
  const history = useHistory()

  return (
    <React.Fragment>
      <br/>
      <div className="container">
        <Formik
          initialValues={{
            headline: "",
            introduction: "",
          }}
          validationSchema={Yup.object().shape({
            headline: Yup.string().required("Headline is required"),
            introduction: Yup.string().required("Introduction is required"),
          })}
          onSubmit={async (values, actions) => {
            //values.countvariable = count
            console.log(values)
            try {
              const file = await ipfs({
                path: "profile.json",
                content: JSON.stringify(values),
              })

              console.log(file)
              console.log(file.cid.string)
              console.log(nearvar.contract)
              await nearvar.contract.create_profile({
                profile_hash: file.cid.string,
              })

              // const content = JSON.stringify(values);
              // const filename = "profile.json"
              // const data = await Ipfsadd(content, filename)
              // await nearvar.contract.create_profile({ profile_hash: data.path.cid.string})
              callUserId()
              setUserIdEmpty(false)
              history.push("/profile")
            } catch (e) {
              console.error(e)
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
              <SubmittingWheel isSubmitting={isSubmitting} />
              <FocusError />
            </Form>
          )}
        </Formik>
      </div>
    </React.Fragment>
  )
}

export default CreateProfile
