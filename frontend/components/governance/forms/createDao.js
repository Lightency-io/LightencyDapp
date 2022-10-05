import React, { useState } from "react";

//Components
import Loader2 from "../../shared/Loading/loader2";

//Styling
import styled from "styled-components";

//Primeng
import { Steps } from "primereact/steps";
import { InputText } from "primereact/inputtext";
import { Button } from "primereact/button";
import { classNames } from "primereact/utils";

const CreateDao = ({
  formik,
  isLoading,
  activeIndex,
  setActiveIndex,
  goNext,
  goPrevious,
}) => {
  const items = [
    { label: "DAO info" },
    { label: "KYC" },
    { label: "Social links" },
    { label: "Deadline" },
    { label: "Summary" },
  ];

  return (
    <>
      <div className="title-container">
        <div className="title">
          <h4>Create a DAO</h4>
        </div>
      </div>
      {!isLoading ? (
        <div>
          <Steps
            model={items}
            activeIndex={activeIndex}
            onSelect={(e) => setActiveIndex(e.index)}
            readOnly={false}
          />
          <div
            className="container"
            style={{ marginTop: "3rem", position: "relative", height: "7rem" }}
          >
            <form onSubmit={formik.handleSubmit}>
              {activeIndex === 0 && (
                <div className="container">
                  <div className="row">
                    <div className="col-md-6">
                      {" "}
                      <span className="p-float-label grid-item">
                        <InputText
                          id="name"
                          value={formik.values.name}
                          onChange={formik.handleChange}
                          onBlur={formik.handleBlur}
                          className={classNames({
                            "p-invalid":
                              formik.touched.name && formik.errors.name,
                          })}
                        />

                        <label
                          htmlFor="name"
                          className={classNames({
                            "p-error":
                              formik.touched.name && formik.errors.name,
                          })}
                        >
                          {formik.touched.name && formik.errors.name
                            ? formik.errors.name
                            : "Name"}
                        </label>
                      </span>
                    </div>
                    <div className="col-md-6 phone">
                      <span className="p-float-label grid-item">
                        <InputText
                          id="purpose"
                          value={formik.values.purpose}
                          onChange={formik.handleChange}
                          onBlur={formik.handleBlur}
                          className={classNames({
                            "p-invalid":
                              formik.touched.purpose && formik.errors.purpose,
                          })}
                        />
                        <label
                          htmlFor="purpose"
                          className={classNames({
                            "p-error":
                              formik.touched.purpose && formik.errors.purpose,
                          })}
                        >
                          {formik.touched.purpose && formik.errors.purpose
                            ? formik.errors.purpose
                            : "Purpose"}
                        </label>
                      </span>
                    </div>
                  </div>
                </div>
              )}

              {activeIndex === 3 && (
                <div className="container">
                  <div className="row">
                    <div className="col-md-4">
                      <span className="p-float-label grid-item"></span>
                    </div>
                  </div>
                </div>
              )}

              {activeIndex === 4 && (
                <div className="container">
                  <div className="row">
                    <div className="col-md-4">
                      {" "}
                      <span className="p-float-label grid-item">
                        <InputText
                          id="daysDuration"
                          value={formik.values.daysDuration}
                          onChange={formik.handleChange}
                          onBlur={formik.handleBlur}
                          className={classNames({
                            "p-invalid":
                              formik.touched.daysDuration &&
                              formik.errors.daysDuration,
                          })}
                        />
                        <label
                          htmlFor="daysDuration"
                          className={classNames({
                            "p-error":
                              formik.touched.daysDuration &&
                              formik.errors.daysDuration,
                          })}
                        >
                          {formik.touched.daysDuration &&
                          formik.errors.daysDuration
                            ? formik.errors.daysDuration
                            : "Days"}
                        </label>
                      </span>
                    </div>
                    <div className="col-md-4">
                      <span className="p-float-label grid-item">
                        <InputText
                          id="hoursDuration"
                          value={formik.values.hoursDuration}
                          onChange={formik.handleChange}
                          onBlur={formik.handleBlur}
                          className={classNames({
                            "p-invalid":
                              formik.touched.hoursDuration &&
                              formik.errors.hoursDuration,
                          })}
                        />
                        <label
                          htmlFor="hoursDuration"
                          className={classNames({
                            "p-error":
                              formik.touched.hoursDuration &&
                              formik.errors.hoursDuration,
                          })}
                        >
                          {formik.touched.hoursDuration &&
                          formik.errors.hoursDuration
                            ? formik.errors.hoursDuration
                            : "Hours"}
                        </label>
                      </span>
                    </div>
                    <div className="col-md-4">
                      <span className="p-float-label grid-item">
                        <InputText
                          id="minuteDuration"
                          value={formik.values.minuteDuration}
                          onChange={formik.handleChange}
                          className={classNames({
                            "p-invalid":
                              formik.touched.minuteDuration &&
                              formik.errors.minuteDuration,
                          })}
                        />
                        <label
                          htmlFor="minuteDuration"
                          className={classNames({
                            "p-error":
                              formik.touched.minuteDuration &&
                              formik.errors.minuteDuration,
                          })}
                        >
                          {formik.touched.minuteDuration &&
                          formik.errors.minuteDuration
                            ? formik.errors.minuteDuration
                            : "Minutes"}
                        </label>
                      </span>
                    </div>
                  </div>
                </div>
              )}
              <div
                className="footer"
                style={{ position: "absolute", right: 0, bottom: 0 }}
              >
                {activeIndex >= 5 && (
                  <Button
                    type="submit"
                    label="Submit"
                    disabled={!formik.isValid || !formik.dirty}
                    className="mt-2"
                  />
                )}
              </div>
            </form>
            {activeIndex < 5 && (
              <div
                className="footer"
                style={{ position: "absolute", right: 0, bottom: 0 }}
              >
                {activeIndex > 0 && (
                  <Button
                    label="Previous "
                    className="p-button-secondary"
                    style={{ marginRight: "1rem" }}
                    onClick={() => goPrevious()}
                  />
                )}
                {activeIndex < 5 && (
                  <Button label="Next" onClick={() => goNext()} />
                )}
              </div>
            )}
          </div>
        </div>
      ) : (
        <>
          <div
            style={{
              textAlign: "center",
              marginTop: "2.5em",
              paddingBottom: "2.5em",
            }}
          >
            <Loader2 />
          </div>
        </>
      )}
    </>
  );
};

const Section = styled.section`
  background-color: black;
  border-radius: 1rem;
  padding: 1rem;
  height: 100%;
  width: 100%;
  color: white;
  @media only screen and (max-width: 550px) {
    width: 23rem;
  }
  .title-container {
    display: flex;
    justify-content: space-between;
    margin-bottom: 1rem;
  }
  .title {
    h1 {
      font-size: 2rem;
      letter-spacing: 0.2rem;
    }
  }
  }
`;
export default CreateDao;
