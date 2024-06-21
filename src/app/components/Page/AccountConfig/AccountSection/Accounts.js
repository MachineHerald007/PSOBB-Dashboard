import React, { useState, useEffect, useContext, useRef } from "react"
import { AccountsContext } from "@/app/page";
import { PlusOutlined } from "@ant-design/icons";
import { Text, Pane } from "evergreen-ui";
import { CenteredPane, HoverPane, AccountPane } from "./styles";
import { useTheme } from "../../../Theme/Theme";
import { ThemeToggler } from "../../../Theme/ThemeToggler";


function getAccounts(setAccounts) {
    invoke("get_accounts")
    .then(res => {
        console.log("Accounts: ", res);
        setAccounts(res);
    })
    .catch(err => {
        console.log(err);
        setAccounts([]);
    }); 
}

export function Accounts({ onAddAccountClick }) {
    const { accounts, setAccounts, getAccounts } = useContext(AccountsContext);
    const [isOverflow, setIsOverflow] = useState(false)
    const containerRef = useRef(null)
    const { theme } = useTheme()

    // Refactor these into styled components, and ALL React Elements
    // that are using inline styles are to be converted into styled elements
    const styles = {
        color: theme === 'light' ? '#43454f' : '#efefef',
    }

    useEffect(() => {
        const container = containerRef.current
        if (container.scrollWidth > container.clientWidth) {
            setIsOverflow(true)
        } else {
            setIsOverflow(false)
        }
        getAccounts(setAccounts)
    }, [])

    return(
        <CenteredPane>
            <Text 
                display="block"
                color={styles.color}
                fontSize={20}
                fontWeight={600}
                marginBottom={32}
                textAlign="center"
            >
                {accounts.length > 0 ? "Select account" : "No accounts available"}
            </Text>
            <Pane
                display="flex"
                height={180}
                maxWidth="100%"
                overflowX={isOverflow ? "scroll" : "hidden"}
                overflowY="hidden"
                ref={containerRef}
            >
                <Pane
                    display="inline-block"
                    marginBottom={14}
                    marginRight={14}
                    position="relative"
                    top={20}
                    onClick={onAddAccountClick}
                >
                    <HoverPane>
                        <PlusOutlined />
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
                    <Pane
                        display="inline-block"
                        textAlign="center"
                        marginLeft={8}
                        key={index}
                    >
                        <AccountPane>
                            <Pane
                                position="relative"
                                overflow="hidden"
                                top={30}
                            >
                                <Text
                                    color={styles.color}
                                    fontSize={14}
                                    fontWeight={600}
                                >
                                    {account.account_name}
                                </Text>
                            </Pane>
                        </AccountPane>
                    </Pane>
                ))}
            </Pane>
        </CenteredPane>
    )
}