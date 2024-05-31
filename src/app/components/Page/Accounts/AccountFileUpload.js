import React, { useState, useMemo, useCallback } from "react"
import { Avatar, Pane, Account, Alert, FileCard, majorScale, FileUploader } from 'evergreen-ui'
import styled, { ThemeProvider } from "styled-components"
import { useTheme } from "../../Theme/Theme"

// Custom rejection reasons
const RejectionReason = {
    OverFileLimit: 'OverFileLimit',
    InvalidMimeType: 'InvalidMimeType',
    FileTooLarge: 'FileTooLarge'
}

const MimeType = {
    psobank: "",
    psoclassicbank: "",
    psochar: ""
}

const AccountPane = styled(Pane)`
    label, p {
        color: ${({ theme }) => (theme.mode === 'light' ? '#474d66' : '#f3f3f3')} !important;
        font-weight: 600;
    }
`;

const AccountFileUploader = styled(FileUploader)`
    background-color: ${({ theme }) => (theme.mode === 'light' ? '#FAFBFF' : '#282A2E')};
    border-color: ${({ theme }) => (theme.mode === 'light' ? '#d8dae5' : '#474d66')};
    
    &:hover {
        background-color: ${({ theme }) => (theme.mode === 'light' ? '#F4F6FA' : '#1f2023')} !important;
        border-color: ${({ theme }) => (theme.mode === 'light' ? '#8f95b2' : '#35394b')} !important;

        .ub-color_5C85FF {
            color: ${({ theme }) => (theme.mode === 'light' ? '#3366FF' : '#6ff3e2')} !important;
        }
        .ub-color_696f8c {
            color: ${({ theme }) => (theme.mode === 'light' ? '#101840' : '#fff')} !important;
        }
    }

    &:focus {
        box-shadow: 0px 0px 0px 2px ${({ theme }) => (theme.mode === 'light' ? '#D6E0FF' : '#282A2E')} !important;
        border-color: ${({ theme }) => (theme.mode === 'light' ? '#ADC2FF' : '#282A2E')} !important;
    }

    span {
        letter-spacing: 0.5px;
        font-weight: 600;
    }

    .ub-color_5C85FF {
        color: ${({ theme }) => (theme.mode === 'light' ? '#3366FF' : '#6ff3e2')};
    }

    .ub-color_696f8c {
        color: ${({ theme }) => (theme.mode === 'light' ? '#101840' : '#fff')};
    }

    .ub-bg-clr_edeff5 {
        background-color: ${({ theme }) => (theme.mode === 'light' ? '#edeff5' : '#202329')};
    }
`;

const AccountFileCard = styled(FileCard)`
    border-color: ${({ theme }) => (theme.mode === 'light' ? '#d8dae5' : '#161616')} !important;

    &:hover {
        cursor: pointer;
        background-color: ${({ theme }) => (theme.mode === 'light' ? '#F4F6FA' : '#1f2023')} !important;
    }

    // trash-icon
    .ub-fill_696f8c {
        fill: ${({ theme }) => (theme.mode === 'light' ? '#696f8c' : '#f3f3f3')};
    }

    // trash icon container
    .ub-bg-clr_F4F5F9_vmhk7m:not([disabled]):hover {
        background-color: ${({ theme }) => (theme.mode === 'light' ? '#dde0e7' : '#dde0e7')} !important;

        &:hover {
            // trash-icon
            .ub-fill_696f8c {
                fill: ${({ theme }) => (theme.mode === 'light' ? '#696f8c' : '#696f8c')};
            }
        }
    }

    .ub-bg-clr_FDF4F4_662z46[aria-invalid='true'] {
        background-color: ${({ theme }) => (theme.mode === 'light' ? '#FAFBFF' : '#282A2E')} !important;
        border-color: #D14343 !important;
    }
`;

const AccountFileCardError = styled(FileCard)`
    border-color: #D14343 !important;
    background-color: ${({ theme }) => (theme.mode === 'light' ? '#FDF4F4' : '#e55353')} !important;

    label, p {
        color: ${({ theme }) => (theme.mode === 'light' ? '#A73636' : '#fff')} !important;
    }

    &:hover {
        cursor: pointer;
        background-color: ${({ theme }) => (theme.mode === 'light' ? '#f7dede' : '#ff0000')} !important;
    }

    // danger icon
    .ub-fill_D14343 {
        fill: ${({ theme }) => (theme.mode === 'light' ? '#D14343' : '#fff')};
    }

    // trash-icon
    .ub-fill_696f8c {
        fill: ${({ theme }) => (theme.mode === 'light' ? '#A73636' : '#fff')};
    }

    // trash icon container
    .ub-bg-clr_F4F5F9_vmhk7m:not([disabled]):hover {
        background-color: ${({ theme }) => (theme.mode === 'light' ? '#D14343' : '#FFF')} !important;

        &:hover {
            // trash-icon
            .ub-fill_696f8c {
                fill: ${({ theme }) => (theme.mode === 'light' ? '#e3e3e3' : '#ff0000')}
            }
        }
    }

    .ub-bg-clr_FDF4F4_662z46[aria-invalid='true'] {
        background-color: ${({ theme }) => (theme.mode === 'light' ? '#FAFBFF' : '#282A2E')} !important;
        border-color: #D14343 !important;
    }

`;

export function AccountFileUpload({ theme }) {
    const acceptedMimeTypes = [MimeType.jpeg, MimeType.pdf, MimeType.psobank, MimeType.psoclassicbank, MimeType.psochar]
    const maxFiles = 32
    const maxSizeInBytes = 50 * 1024 ** 2 // 50 MB
    const [files, setFiles] = useState([])
    const [fileRejections, setFileRejections] = useState([])

    const values = useMemo(() => [...files, ...fileRejections.map((fileRejection) => fileRejection.file)], [
        files,
        fileRejections,
    ])

    const handleRemove = useCallback(
        (file) => {
            const updatedFiles = files.filter((existingFile) => existingFile !== file)
            const updatedFileRejections = fileRejections.filter((fileRejection) => fileRejection.file !== file)

            const { accepted, rejected } = rebaseFiles(
                [...updatedFiles, ...updatedFileRejections.map((fileRejection) => fileRejection.file)],
                { acceptedMimeTypes, maxFiles, maxSizeInBytes }
            )

            setFiles(accepted)
            setFileRejections(rejected)
        },
        [acceptedMimeTypes, files, fileRejections, maxFiles, maxSizeInBytes]
    )

    const rebaseFiles = (allFiles, { acceptedMimeTypes, maxFiles, maxSizeInBytes }) => {
        const accepted = []
        const rejected = []

        allFiles.forEach((file) => {
            if (accepted.length < maxFiles) {
                if (acceptedMimeTypes.includes(file.type) && file.size <= maxSizeInBytes) {
                    accepted.push(file)
                } else {
                    rejected.push({
                        file,
                        reason: !acceptedMimeTypes.includes(file.type)
                            ? RejectionReason.InvalidMimeType
                            : RejectionReason.FileTooLarge
                    })
                }
            } else {
                rejected.push({
                    file,
                    reason: RejectionReason.OverFileLimit
                })
            }
        })

        return { accepted, rejected }
    }

    const fileCountOverLimit = files.length + fileRejections.length - maxFiles
    const fileCountError = `You can upload up to ${maxFiles} files. Please remove ${fileCountOverLimit} ${
        fileCountOverLimit === 1 ? 'file' : 'files'
    }.`

    return (
        <AccountPane maxWidth={654}>
            <AccountFileUploader
                width={654}
                minWidth={400}
                acceptedMimeTypes={acceptedMimeTypes}
                label="Upload Files"
                description="You can upload .psobank, .psoclassicbank, .psochar file formats only."
                disabled={files.length + fileRejections.length >= maxFiles}
                maxSizeInBytes={maxSizeInBytes}
                maxFiles={maxFiles}
                onAccepted={setFiles}
                onRejected={setFileRejections}
                renderFile={(file, index) => {
                    const { name, size, type } = file
                    const renderFileCountError = index === 0 && fileCountOverLimit > 0
                    const fileRejection = fileRejections.find(
                        (fileRejection) => fileRejection.file === file && fileRejection.reason !== RejectionReason.OverFileLimit
                    )
                    const { message } = fileRejection || {}

                    return (
                        <React.Fragment key={`${file.name}-${index}`}>
                            {renderFileCountError && <Alert intent="danger" marginBottom={majorScale(2)} title={fileCountError} />}
                            {
                                fileRejection != null ?
                                <AccountFileCardError 
                                    isInvalid={fileRejection != null}
                                    name={name}
                                    onRemove={() => handleRemove(file)}
                                    sizeInBytes={size}
                                    type={type}
                                    validationMessage={message}
                                />
                                :
                                <AccountFileCard
                                    isInvalid={fileRejection != null}
                                    name={name}
                                    onRemove={() => handleRemove(file)}
                                    sizeInBytes={size}
                                    type={type}
                                    validationMessage={message}
                                />
                            }
                        </React.Fragment>
                    )
                }}
                values={values}
            />
        </AccountPane>
    )
}