import React, { useState, useContext} from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { useHistory, useParams  } from "react-router-dom"
import ipfs from "../../commons/ipfs"
import { NearContext } from "../../commons/context/NearContext"
import { FocusError, SubmittingWheel } from "../../commons/FocusWheel"





function DrawJuror(props) {
    // const [count, setCount] = useState(0);
    const { rid } = useParams()
    console.log(rid, "rid")
    let history = useHistory();
    let {nearvar} = useContext(NearContext)
    const [errorThrow, setErrorThrow] = useState(false);

    return (
        <React.Fragment>
      <div className="container">
        <Formik 
          initialValues= {{
             length: "", 
             }}
          validationSchema = {Yup.object().shape({
             length: Yup.number().required("length is required"),
            
          })}
          onSubmit={async (values, actions) => {
          	try {
              console.log("try")
          	//   values.countvariable = count
              const data = await nearvar.contract.draw_jurors({
                review_id: rid.toString(),
                length: parseInt(values.length),
              })
              actions.setSubmitting(false)
              // console.log(data)
              // history.push(`/thankyou${data.mutationoutputname}`)
              history.goBack()
          	} catch (e) {
              console.error(e)
              setErrorThrow(e.message)
            }
            
              
          }}        
        >
         {({ handleSubmit, handleBlur, handleChange, errors, touched, isValid, isSubmitting, values, setFieldValue, validateForm }) => (
             <Form onSubmit={handleSubmit}>
             {errorThrow && <p>{errorThrow}</p>}
              
                <div className="form-group">
                <label htmlFor="length">length</label>
                {touched.length && errors.length && <p className="alert alert-danger">{errors.length}</p>}
                
                <Field name="length" className="form-control" />
                
                </div>
              
              <div className="text-center">
                <button type="submit" className="btn btn-primary" disabled={isSubmitting}>
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



export default DrawJuror

