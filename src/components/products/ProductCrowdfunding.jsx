import React, { useState, useContext } from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { useHistory, useParams } from "react-router-dom"
import { NearContext } from "../../commons/context/NearContext"
import ipfs from "../../commons/ipfs"
import { FocusError, SubmittingWheel } from "../../commons/FocusWheel"

function ProductCrowdfunding(props) {
  // const [count, setCount] = useState(0);
  const { pid } = useParams()
  let history = useHistory()
  let { nearvar, reloadBalance} = useContext(NearContext)
  const [errorThrow, setErrorThrow] = useState(false)

  return (
    <React.Fragment>
      <div className="container">
        <Formik
          initialValues={{
            bounty: "",
          }}
          validationSchema={Yup.object().shape({
            bounty: Yup.number().required("bounty is required"),
          })}
          onSubmit={async (values, actions) => {
            try {
              //   values.countvariable = count
              await nearvar.contract.add_product_crowdfunding({
                bounty: values.bounty.toString(),
                product_id: pid.toString(),                
              })
              actions.setSubmitting(false)
              // console.log(data)
              // history.push(`/thankyou${data.mutationoutputname}`)
              reloadBalance()
              history.goBack()
            } catch (e) {
              console.error(e)
              setErrorThrow(e.message)
            }
          }}
        >
          {({
            handleSubmit,
            handleBlur,
            handleChange,
            errors,
            touched,
            isValid,
            isSubmitting,
            values,
            setFieldValue,
            validateForm,
          }) => (
            <Form onSubmit={handleSubmit}>
              {errorThrow && <p>{errorThrow}</p>}

              <div className="form-group">
                <label htmlFor="bounty">Fund it</label>
                {touched.bounty && errors.bounty && (
                  <p className="alert alert-danger">{errors.bounty}</p>
                )}

                <Field name="bounty" className="form-control" />
              </div>

              <div className="text-center">
                <button
                  type="submit"
                  className="btn btn-primary"
                  disabled={isSubmitting}
                >
                  Submit Form
                </button>
              </div>
              <SubmittingWheel isSubmitting={isSubmitting} />
              <FocusError />
            </Form>
          )}
        </Formik>
      </div>
    </React.Fragment>
  )
}

export default ProductCrowdfunding
