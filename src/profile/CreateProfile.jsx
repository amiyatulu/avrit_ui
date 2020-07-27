import React, { useState, useContext } from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { FocusError, SubmittingWheel } from "../commons/FocusWheel"
import { NearContext } from "../context/NearContext"
import ipfs from "../commons/ipfs"
import ProfileForm from './ProfileForm';

function CreateProfile() {
  const [count, setCount] = useState(0)
  let nearvar = useContext(NearContext)
  console.log(nearvar)

  return (
    <React.Fragment>
      <div className="container">
        <Formik
          initialValues={{
            headline: "",
            introduction: "",
            details: "",
            youAre: "",
            skills: "",
          }}
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
            try{
              const file = await ipfs.add({
                path: "profile.json",
                content: JSON.stringify(values),
              })
              
              console.log(file)
              console.log(file.cid.string)
              console.log(nearvar.contract)
              await nearvar.contract.create_profile({ profile_hash: file.cid.string})

              const data = await nearvar.contract.get_profile_hash();
              console.log("data", data)
              
            } catch(e) {
              console.error(e);
            }

            
            
            // actions.setSubmitting(false)
            // console.log(data)
          }}
        >
          {({ handleSubmit, handleBlur, handleChange, errors, touched, isSubmitting, values, setFieldValue, validateForm }) => (
            <Form onSubmit={handleSubmit}>
              <ProfileForm errors={errors} touched={touched}/>
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
