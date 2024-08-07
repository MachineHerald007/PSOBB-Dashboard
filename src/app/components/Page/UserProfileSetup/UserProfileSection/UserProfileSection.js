import React, { useState, useContext, useCallback } from "react";
import { AppContext } from "@/app/page";
import { Pane } from "evergreen-ui";
import { UserOutlined, DiscordOutlined } from '@ant-design/icons';
import { ProfilePictureUpload } from "./ProfilePictureUpload/ProfilePictureUpload";
import { TextWrapper, InputWrapper } from "./styles";
import { CreateUserButton } from "./CreateUserButton/CreateUserButton";
import { useTheme } from "../../../Theme/Theme";
import { invoke } from "@tauri-apps/api/tauri";

export function UserProfileSection() {
    const { setUser, getUser } = useContext(AppContext);
    const theme = useTheme();
    const [profilePicture, setProfilePicture] = useState("");
    const [profileName, setProfileName] = useState("");
    const [discordUsername, setDiscordUsername] = useState("");

    const handleProfilePictureChange = useCallback((picture) => {
        setProfilePicture(picture);
    }, []);

    const handleCreateUser = useCallback(() => {
        const user = {
            profile_name: profileName,
            discord_username: discordUsername,
            profile_picture: profilePicture,
        };

        invoke("create_user", { user })
            .then(() => getUser(setUser))
            .catch(err => console.log(err));
    }, [profileName, discordUsername, profilePicture, getUser, setUser]);

    return (
        <Pane>
            <Pane theme={theme} width={270}>
                <Pane>
                    <ProfilePictureUpload onChange={handleProfilePictureChange} />
                </Pane>
                <Pane marginBottom={12}>
                    <InputWrapper
                        value={profileName}
                        onChange={e => setProfileName(e.target.value)}
                        placeholder="Profile Name"
                        prefix={<UserOutlined style={{ marginLeft: "4px" }} />}
                    />
                </Pane>
                <Pane marginBottom={12}>
                    <InputWrapper
                        value={discordUsername}
                        onChange={e => setDiscordUsername(e.target.value)}
                        placeholder="Discord Username"
                        prefix={<DiscordOutlined style={{ marginLeft: "4px" }} />}
                    />
                </Pane>
                <Pane textAlign="right">
                    <CreateUserButton onClick={handleCreateUser} />
                </Pane>
            </Pane>
        </Pane>
    );
}