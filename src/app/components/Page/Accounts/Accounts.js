import React, { useState, useEffect, useRef } from "react"
import {
    Avatar,
    AddIcon,
    PlusIcon,
    Text,
    Pane
} from "evergreen-ui"
import styled from "styled-components"
import { useTheme } from "../../Theme/Theme"
import { ThemeToggler } from "../../Theme/ThemeToggler"
import { AccountSetup } from "./AccountSetup"

function getAccounts() {
    return [
        {
            name: "machineherald",
            profile_pic: "https://i.imgur.com/2swHEdn_d.webp?maxwidth=760&fidelity=grand",
            characters: [],
            shared_bank: null
        },
        {
            name: "herald001",
            profile_pic: "https://i.imgur.com/w9tCRCF_d.webp?maxwidth=760&fidelity=grand",
            characters: [],
            shared_bank: null
        }
    ]
}

const HoverPane = styled(Pane)`
    width: fit-content;
    margin-bottom: 12px;
    border-radius: 50px;
    transition: background-color 0.3s ease;
    position: relative;
    left: 8px;

    &:hover {
        cursor: pointer;
        background-color: grey;
        color: white; /* Optional: change text color on hover */
    }
`;

const ProfileAvatar = styled(Avatar)`
    &:hover {
        cursor: pointer;
        opacity: 0.5;
    }
`;

const CenteredPane = styled(Pane)`
    height: fit-content;
    width: fit-content;
    maxWidth: 80%;
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    margin: auto;
    overflow: hidden;

    .ub-lft_12px {
        left: 0 !important;
    }
`;

// .ub-lft_12px
const AccountSection = ({ accounts }) => {
    const { theme } = useTheme()
    const [isOverflow, setIsOverflow] = useState(false)
    const containerRef = useRef(null)

    const styles = {
        color: theme === 'light' ? '#101840' : '#fff',
    }

    const hpane_styles = {
        background: theme === 'light' ? '#d8dae5' : '#474649',
        color: theme === 'light' ? '#8f95b2' : '#edeff5',
    }

    useEffect(() => {
        const container = containerRef.current
        if (container.scrollWidth > container.clientWidth) {
            setIsOverflow(true)
        } else {
            setIsOverflow(false)
        }
    }, [accounts])

    return (
        <CenteredPane>
            <Text 
                display="block"
                color={styles.color}
                fontSize={20}
                marginBottom={32}
            >
                {accounts.length > 0 ? "Select account" : "No accounts available"}
            </Text>
            <Pane
                display="flex"
                maxWidth="100%"
                overflowX={isOverflow ? "scroll" : "hidden"}
                overflowY="hidden"
                ref={containerRef}
            >
                <Pane
                    display="inline-block"
                    marginBottom={16}
                    position="relative"
                    top={8}
                >
                    <HoverPane
                        backgroundColor={hpane_styles.background}
                        color={hpane_styles.color}
                        padding={16}
                    >
                        <PlusIcon size={32} position="relative" top={2}/>
                    </HoverPane>
                    <Text
                        color={styles.color}
                        fontSize={14}
                        fontWeight={600}
                        position="relative"
                        left={12}
                        top={6}
                    >
                        Add Account
                    </Text>
                </Pane>
                {accounts.map((account, index) => (
                    <Pane display="inline-block" textAlign="center" marginLeft={36} key={index}>
                        <ProfileAvatar
                            marginBottom={16}
                            src={account.profile_pic}
                            size={80} 
                        />
                        <Pane>
                            <Text
                                color={styles.color}
                                fontSize={14}
                                fontWeight={600}
                                position="relative"
                                left={12}
                            >
                                {account.name}
                            </Text>
                        </Pane>
                    </Pane>
                ))}
            </Pane>
        </CenteredPane>
    )
}

export function Accounts() {
    const { theme } = useTheme()
    const [accounts, setAccounts] = useState([])

    useEffect(() => {
        const accounts = getAccounts()
        setAccounts(accounts)
    }, [])

    const styles = {
        background: theme === 'light' ? '#FFFFFF' : '#24252B',
        color: theme === 'light' ? '#101840' : '#fff',
        height: "100vh"
    }

    return (
        <Pane style={styles}>
            <ThemeToggler />
            {/* <AccountSection accounts={accounts}/> */}
            <AccountSetup />
        </Pane>
    )
}