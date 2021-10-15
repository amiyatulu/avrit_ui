import React, { useState, useContext } from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { useHistory, useParams } from "react-router-dom"
import ipfs from "../../commons/ipfs"
import { NearContext } from "../../commons/context/NearContext"
import { FocusError, SubmittingWheel } from "../../commons/FocusWheel"

function PDisapprovalProductIncentives(props) {
  // const [count, setCount] = useState(0);
  const { pid } = useParams()
  // console.log(rid, "rid")
  let history = useHistory()
  let { nearvar, reloadBalance } = useContext(NearContext)
  const [errorThrow, setErrorThrow] = useState(false)

  return (
    <React.Fragment>
      <div className="container">
        <Formik
          initialValues={{}}
          validationSchema={Yup.object().shape({})}
          onSubmit={async (values, actions) => {
            try {
              await nearvar.contract.disapproval_product_incentives({
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
              <div className="container">
                <div className="text-center">
                  <br/><br/>
                  <p className="badge badge-secondary mr-3">
                    Draw after disapproval schelling game has finished
                  </p><br/>
                  {errorThrow && <p>{errorThrow}</p>}<br/>
                  <button
                    type="submit"
                    className="btn btn-primary"
                    disabled={isSubmitting}
                  >
                    Draw incentives for Disapproval of Product
                  </button>
                </div>
                <SubmittingWheel isSubmitting={isSubmitting} />
                <FocusError />
              </div>
            </Form>
          )}
        </Formik>
      </div>
    </React.Fragment>
  )
}

export default PDisapprovalProductIncentives
