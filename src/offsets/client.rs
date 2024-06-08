pub const dwLocalPlayerPawn: usize = 0x17C9468;

pub mod C_BaseEntity {
    pub const m_fFlags: usize = 0x3CC; // uint32
}

pub mod C_BasePlayerPawn {
    pub const m_pCameraServices: usize = 0x1130; // CPlayer_CameraServices*
}

pub mod C_CSPlayerPawnBase {
    pub const m_iIDEntIndex: usize = 0x13A8; // CEntityIndex
}

pub mod C_CSPlayerPawn {
    pub const m_bIsScoped: usize = 0x2290; // bool
}

pub mod CCSPlayerBase_CameraServices {
    pub const m_iFOV: usize = 0x210; // uint32
}
