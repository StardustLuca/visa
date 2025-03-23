use thiserror::Error;
use visa_bindings::*;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Visa(VisaError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Invalid string")]
    InvalidString,
    #[error("Failed to parse identification: {0}")]
    InvalidIdentification(String),
    #[error("Instrument not found")]
    InstrumentNotFound,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, PartialOrd)]
#[repr(i32)]
pub enum VisaError {
    /// Unknown system error (miscellaneous error).
    #[error("Unknown system error (miscellaneous error).")]
    Unknown = VI_ERROR_SYSTEM_ERROR,
    /// The given session or object reference is invalid.
    #[error("The given session or object reference is invalid.")]
    InvalidObject = VI_ERROR_INV_OBJECT,
    /// Specified type of lock cannot be obtained or specified operation cannot be performed, because the resource is locked.
    #[error(
        "Specified type of lock cannot be obtained or specified operation cannot be performed, because the resource is locked."
    )]
    ResourceLocked = VI_ERROR_RSRC_LOCKED,
    /// Invalid expression specified for search.
    #[error("Invalid expression specified for search.")]
    InvalidExpression = VI_ERROR_INV_EXPR,
    /// Insufficient location information or the device or resource is not present in the system.
    #[error(
        "Insufficient location information or the device or resource is not present in the system."
    )]
    ResourceNotFound = VI_ERROR_RSRC_NFOUND,
    /// Invalid resource reference specified. Parsing error.
    #[error("Invalid resource reference specified. Parsing error.")]
    InvalidResourceName = VI_ERROR_INV_RSRC_NAME,
    /// Invalid access mode.
    #[error("Invalid access mode.")]
    InvalidAccessMode = VI_ERROR_INV_ACC_MODE,
    /// Timeout expired before operation completed.
    #[error("Timeout expired before operation completed.")]
    Timeout = VI_ERROR_TMO,
    /// Unable to deallocate the previously allocated data structures corresponding to this session or object reference.
    #[error(
        "Unable to deallocate the previously allocated data structures corresponding to this session or object reference."
    )]
    DeallocationFailed = VI_ERROR_CLOSING_FAILED,
    /// Specified degree is invalid.
    #[error("Specified degree is invalid.")]
    InvalidDegree = VI_ERROR_INV_DEGREE,
    /// Specified job identifier is invalid.
    #[error("Specified job identifier is invalid.")]
    InvalidJobId = VI_ERROR_INV_JOB_ID,
    /// The specified attribute is not defined or supported by the referenced session, event, or find list.
    #[error(
        "The specified attribute is not defined or supported by the referenced session, event, or find list."
    )]
    AttributeNotSupported = VI_ERROR_NSUP_ATTR,
    /// The specified state of the attribute is not valid, or is not supported as defined by the session, event, or find list.
    #[error(
        "The specified state of the attribute is not valid, or is not supported as defined by the session, event, or find list."
    )]
    AttributeStateNotSupported = VI_ERROR_NSUP_ATTR_STATE,
    /// The specified attribute is Read Only.
    #[error("The specified attribute is Read Only.")]
    AttributeReadOnly = VI_ERROR_ATTR_READONLY,
    /// The specified type of lock is not supported by this resource.
    #[error("The specified type of lock is not supported by this resource.")]
    InvalidLockType = VI_ERROR_INV_LOCK_TYPE,
    /// The access key to the resource associated with this session is invalid.
    #[error("The access key to the resource associated with this session is invalid.")]
    InvalidAccessKey = VI_ERROR_INV_ACCESS_KEY,
    /// Specified event type is not supported by the resource.
    #[error("Specified event type is not supported by the resource.")]
    InvalidEvent = VI_ERROR_INV_EVENT,
    /// Invalid mechanism specified.
    #[error("Invalid mechanism specified.")]
    InvalidMechanism = VI_ERROR_INV_MECH,
    /// A handler is not currently installed for the specified event.
    #[error("A handler is not currently installed for the specified event.")]
    HandlerNotInstalled = VI_ERROR_HNDLR_NINSTALLED,
    /// The given handler reference is invalid.
    #[error("The given handler reference is invalid.")]
    InvalidHandlerReference = VI_ERROR_INV_HNDLR_REF,
    /// Specified event context is invalid.
    #[error("Specified event context is invalid.")]
    InvalidContext = VI_ERROR_INV_CONTEXT,
    /// The event queue for the specified type has overflowed (usually due to previous events not having been closed).
    #[error(
        "The event queue for the specified type has overflowed (usually due to previous events not having been closed)."
    )]
    QueueOverflow = VI_ERROR_QUEUE_OVERFLOW,
    /// The session must be enabled for events of the specified type in order to receive them.
    #[error(
        "The session must be enabled for events of the specified type in order to receive them."
    )]
    SessionNotEnabled = VI_ERROR_NENABLED,
    /// The operation was aborted.
    #[error("The operation was aborted.")]
    OperationAborted = VI_ERROR_ABORT,
    /// Violation of raw write protocol occurred during transfer.
    #[error("Violation of raw write protocol occurred during transfer.")]
    RawWriteProtocolViolation = VI_ERROR_RAW_WR_PROT_VIOL,
    /// Violation of raw read protocol occurred during transfer.
    #[error("Violation of raw read protocol occurred during transfer.")]
    RawReadProtocolViolation = VI_ERROR_RAW_RD_PROT_VIOL,
    /// Device reported an output protocol error during transfer.
    #[error("Device reported an output protocol error during transfer.")]
    OutputProtocolViolation = VI_ERROR_OUTP_PROT_VIOL,
    /// Device reported an input protocol error during transfer.
    #[error("Device reported an input protocol error during transfer.")]
    InputProtocolViolation = VI_ERROR_INP_PROT_VIOL,
    /// Bus error occurred during transfer.
    #[error("Bus error occurred during transfer.")]
    Bus = VI_ERROR_BERR,
    /// Unable to queue the asynchronous operation because there is already an operation in progress.
    #[error(
        "Unable to queue the asynchronous operation because there is already an operation in progress."
    )]
    OperationAlreadyInProgress = VI_ERROR_IN_PROGRESS,
    /// Unable to start operation because setup is invalid (due to attributes being set to an inconsistent state).
    #[error(
        "Unable to start operation because setup is invalid (due to attributes being set to an inconsistent state)."
    )]
    InvalidSetup = VI_ERROR_INV_SETUP,
    /// Unable to queue asynchronous operation (usually due to the I/O completion event not being enabled or insufficient space in the session's queue).
    #[error(
        "Unable to queue asynchronous operation (usually due to the I/O completion event not being enabled or insufficient space in the session's queue)."
    )]
    Queue = VI_ERROR_QUEUE_ERROR,
    /// Insufficient system resources to perform necessary memory allocation.
    #[error("Insufficient system resources to perform necessary memory allocation.")]
    Allocation = VI_ERROR_ALLOC,
    /// Invalid buffer mask specified.
    #[error("Invalid buffer mask specified.")]
    InvalidBufferMask = VI_ERROR_INV_MASK,
    /// Could not perform operation because of I/O error.
    #[error("Could not perform operation because of I/O error.")]
    Io = VI_ERROR_IO,
    /// A format specifier in the format string is invalid.
    #[error("A format specifier in the format string is invalid.")]
    InvalidFormatSpecifier = VI_ERROR_INV_FMT,
    /// A format specifier in the format string is not supported.
    #[error("A format specifier in the format string is not supported.")]
    FormatSpecifierNotSupported = VI_ERROR_NSUP_FMT,
    /// The specified trigger line is currently in use.
    #[error("The specified trigger line is currently in use.")]
    LineInUse = VI_ERROR_LINE_IN_USE,
    /// The specified mode is not supported by this VISA implementation.
    #[error("The specified mode is not supported by this VISA implementation.")]
    ModeNotSupported = VI_ERROR_NSUP_MODE,
    /// Service request has not been received for the session.
    #[error("Service request has not been received for the session.")]
    ServiceRequestNotReceived = VI_ERROR_SRQ_NOCCURRED,
    /// Invalid address space specified.
    #[error("Invalid address space specified.")]
    InvalidAddressSpace = VI_ERROR_INV_SPACE,
    /// Invalid offset specified.
    #[error("Invalid offset specified.")]
    InvalidOffset = VI_ERROR_INV_OFFSET,
    /// Invalid source or destination width specified.
    #[error("Invalid source or destination width specified.")]
    InvalidWidth = VI_ERROR_INV_WIDTH,
    /// Specified offset is not accessible from this hardware.
    #[error("Specified offset is not accessible from this hardware.")]
    OffsetNotSupported = VI_ERROR_NSUP_OFFSET,
    /// Cannot support source and destination widths that are different.
    #[error("Cannot support source and destination widths that are different.")]
    DifferentWidthNotSupported = VI_ERROR_NSUP_VAR_WIDTH,
    /// The specified session is not currently mapped.
    #[error("The specified session is not currently mapped.")]
    SessionNotMapped = VI_ERROR_WINDOW_NMAPPED,
    /// The previous response is still pending, causing a multiple query error.
    #[error("The previous response is still pending, causing a multiple query error.")]
    PreviousResponsePending = VI_ERROR_RESP_PENDING,
    /// No Listeners condition is detected (both NRFD and NDAC are deasserted).
    #[error("No Listeners condition is detected (both NRFD and NDAC are deasserted).")]
    NoListeners = VI_ERROR_NLISTENERS,
    /// The interface associated with this session is not currently the controller in charge.
    #[error(
        "The interface associated with this session is not currently the controller in charge."
    )]
    NotControllerInCharge = VI_ERROR_NCIC,
    /// The interface associated with this session is not the system controller.
    #[error("The interface associated with this session is not the system controller.")]
    NotSystemController = VI_ERROR_NSYS_CNTLR,
    /// The given session or object reference does not support this operation.
    #[error("The given session or object reference does not support this operation.")]
    OperationNotSupported = VI_ERROR_NSUP_OPER,
    /// An interrupt is still pending from a previous call.
    #[error("An interrupt is still pending from a previous call.")]
    PreviousInterruptPending = VI_ERROR_INTR_PENDING,
    /// A parity error occurred during transfer.
    #[error("A parity error occurred during transfer.")]
    Parity = VI_ERROR_ASRL_PARITY,
    /// A framing error occurred during transfer.
    #[error("A framing error occurred during transfer.")]
    Framing = VI_ERROR_ASRL_FRAMING,
    /// An overrun error occurred during transfer. A character was not read from the hardware before the next character arrived.
    #[error(
        "An overrun error occurred during transfer. A character was not read from the hardware before the next character arrived."
    )]
    Overrun = VI_ERROR_ASRL_OVERRUN,
    /// The path from trigSrc to trigDest is not currently mapped.
    #[error("The path from trigSrc to trigDest is not currently mapped.")]
    TriggerNotMapped = VI_ERROR_TRIG_NMAPPED,
    /// The specified offset is not properly aligned for the access width of the operation.
    #[error("The specified offset is not properly aligned for the access width of the operation.")]
    OffsetNotAligned = VI_ERROR_NSUP_ALIGN_OFFSET,
    /// A specified user buffer is not valid or cannot be accessed for the required size.
    #[error("A specified user buffer is not valid or cannot be accessed for the required size.")]
    InvalidUserBuffer = VI_ERROR_USER_BUF,
    /// The resource is valid, but VISA cannot currently access it.
    #[error("The resource is valid, but VISA cannot currently access it.")]
    ResourceBusy = VI_ERROR_RSRC_BUSY,
    /// Specified width is not supported by this hardware.
    #[error("Specified width is not supported by this hardware.")]
    WidthNotSupported = VI_ERROR_NSUP_WIDTH,
    /// The value of some parameter—which parameter is not known—is invalid.
    #[error("The value of some parameter—which parameter is not known—is invalid.")]
    InvalidParameter = VI_ERROR_INV_PARAMETER,
    /// The protocol specified is invalid.
    #[error("The protocol specified is invalid.")]
    InvalidProtocol = VI_ERROR_INV_PROT,
    /// Invalid size of window specified.
    #[error("Invalid size of window specified.")]
    InvalidSize = VI_ERROR_INV_SIZE,
    /// The specified session currently contains a mapped window.
    #[error("The specified session currently contains a mapped window.")]
    MappedWindow = VI_ERROR_WINDOW_MAPPED,
    /// The given operation is not implemented.
    #[error("The given operation is not implemented.")]
    OperationNotImplemented = VI_ERROR_NIMPL_OPER,
    /// Invalid length specified.
    #[error("Invalid length specified.")]
    InvalidLength = VI_ERROR_INV_LENGTH,
    /// The specified mode is invalid.
    #[error("The specified mode is invalid.")]
    InvalidMode = VI_ERROR_INV_MODE,
    /// The current session did not have any lock on the resource.
    #[error("The current session did not have any lock on the resource.")]
    SessionNotLocked = VI_ERROR_SESN_NLOCKED,
    /// The device does not export any memory.
    #[error("The device does not export any memory.")]
    NoMemorySupport = VI_ERROR_MEM_NSHARED,
    /// A code library required by VISA could not be located or loaded.
    #[error("A code library required by VISA could not be located or loaded.")]
    LibraryNotFound = VI_ERROR_LIBRARY_NFOUND,
    /// The interface cannot generate an interrupt on the requested level or with the requested statusID value.
    #[error(
        "The interface cannot generate an interrupt on the requested level or with the requested statusID value."
    )]
    InterruptNotSupported = VI_ERROR_NSUP_INTR,
    /// The value specified by the line parameter is invalid.
    #[error("The value specified by the line parameter is invalid.")]
    InvalidLine = VI_ERROR_INV_LINE,
    /// An error occurred while trying to open the specified file. Possible reasons include an invalid path or lack of access rights.
    #[error(
        "An error occurred while trying to open the specified file. Possible reasons include an invalid path or lack of access rights."
    )]
    FileAccess = VI_ERROR_FILE_ACCESS,
    /// An error occurred while performing I/O on the specified file.
    #[error("An error occurred while performing I/O on the specified file.")]
    IoFile = VI_ERROR_FILE_IO,
    /// One of the specified lines (trigSrc or trigDest) is not supported by this VISA implementation, or the combination of lines is not a valid mapping.
    #[error(
        "One of the specified lines (trigSrc or trigDest) is not supported by this VISA implementation, or the combination of lines is not a valid mapping."
    )]
    LineNotSupported = VI_ERROR_NSUP_LINE,
    /// The specified mechanism is not supported for the given event type.
    #[error("The specified mechanism is not supported for the given event type.")]
    MechanismNotSupported = VI_ERROR_NSUP_MECH,
    /// The interface type is valid but the specified interface number is not configured.
    #[error("The interface type is valid but the specified interface number is not configured.")]
    InterfaceNumberNotConfigured = VI_ERROR_INTF_NUM_NCONFIG,
    /// The connection for the given session has been lost.
    #[error("The connection for the given session has been lost.")]
    ConnectionLost = VI_ERROR_CONN_LOST,
    /// The remote machine does not exist or is not accepting any connections.
    #[error("The remote machine does not exist or is not accepting any connections.")]
    MachineNotAvailable = VI_ERROR_MACHINE_NAVAIL,
    /// Access to the resource or remote machine is denied. This is due to a lack of sufficient privileges for the current user or machine.
    #[error(
        "Access to the resource or remote machine is denied. This is due to a lack of sufficient privileges for the current user or machine."
    )]
    NoPermission = VI_ERROR_NPERMISSION,
}

#[repr(u32)]
pub enum Status {
    /// Operation completed successfully
    OperationCompleted = VI_SUCCESS,
    /// Specified event is already enabled for at least one of the specified mechanisms.
    EventAlreadyEnabled = VI_SUCCESS_EVENT_EN,
    /// Specified event is already disabled for at least one of the specified mechanisms.
    EventAlreadyDisabled = VI_SUCCESS_EVENT_DIS,
    /// Operation completed successfully, but queue was already empty.
    OperationCompletedQueueEmpty = VI_SUCCESS_QUEUE_EMPTY,
    /// The specified termination character was read.
    TerminationCharacterRead = VI_SUCCESS_TERM_CHAR,
    /// The number of bytes read is equal to the input count.
    BytesReadEqualInput = VI_SUCCESS_MAX_CNT,
    /// The event returned is valid.  
    /// One or more events that occurred have not been raised because
    /// there was no room available on the queue at the time of their occurrence.  
    /// This could happen because VI_ATTR_MAX_QUEUE_LENGTH is not set to a large enough value for your application
    /// and/or events are coming in faster than you are servicing them.
    QueueOverflow = VI_WARN_QUEUE_OVERFLOW,
    /// The specified configuration either does not exist or could not be loaded; using VISA-specified defaults.
    ConfigurationNotLoaded = VI_WARN_CONFIG_NLOADED,
    /// Session opened successfully, but the device at the specified address is not responding.
    DeviceNotResponding = VI_SUCCESS_DEV_NPRESENT,
    /// The path from trigSrc to trigDest is already mapped.
    TriggerAlreadyMapped = VI_SUCCESS_TRIG_MAPPED,
    /// Wait terminated successfully on receipt of an event notification.  
    /// There is still at least one more event occurrence of the requested type(s) available for this session.
    QueueNotEmpty = VI_SUCCESS_QUEUE_NEMPTY,
    /// The specified object reference is uninitialized.
    NullObject = VI_WARN_NULL_OBJECT,
    /// Although the specified state of the attribute is valid, it is not supported by this resource implementation.
    AttributeStateNotSupported = VI_WARN_NSUP_ATTR_STATE,
    /// The status code passed to the operation could not be interpreted.
    Unknown = VI_WARN_UNKNOWN_STATUS,
    /// The specified buffer is not sup=ported.
    BufferNotSupported = VI_WARN_NSUP_BUF,
    /// Event handled successfully. Do not invoke any other handlers on this session for this event.
    DoNotInvokeEvent = VI_SUCCESS_NCHAIN,
    /// Operation completed successfully, and this session has nested shared locks.
    NestedSharedLocks = VI_SUCCESS_NESTED_SHARED,
    /// Operation completed successfully, and this session has nested exclusive locks.
    NestedExclusiveLocks = VI_SUCCESS_NESTED_EXCLUSIVE,
    /// Asynchronous operation request was actually performed synchronously.
    AsynchronousOperationHandledSynchronously = VI_SUCCESS_SYNC,
    /// The operation succeeded, but a lower level driver did not implement the extended functionality.
    ExtendedFunctionNotImplemented = VI_WARN_EXT_FUNC_NIMPL,
}

impl TryFrom<ViStatus> for VisaError {
    type Error = ();

    fn try_from(value: ViStatus) -> std::result::Result<Self, Self::Error> {
        match value {
            VI_ERROR_SYSTEM_ERROR => Ok(Self::Unknown),
            VI_ERROR_INV_OBJECT => Ok(Self::InvalidObject),
            VI_ERROR_RSRC_LOCKED => Ok(Self::ResourceLocked),
            VI_ERROR_INV_EXPR => Ok(Self::InvalidExpression),
            VI_ERROR_RSRC_NFOUND => Ok(Self::ResourceNotFound),
            VI_ERROR_INV_RSRC_NAME => Ok(Self::InvalidResourceName),
            VI_ERROR_INV_ACC_MODE => Ok(Self::InvalidAccessMode),
            VI_ERROR_TMO => Ok(Self::Timeout),
            VI_ERROR_CLOSING_FAILED => Ok(Self::DeallocationFailed),
            VI_ERROR_INV_DEGREE => Ok(Self::InvalidDegree),
            VI_ERROR_INV_JOB_ID => Ok(Self::InvalidJobId),
            VI_ERROR_NSUP_ATTR => Ok(Self::AttributeNotSupported),
            VI_ERROR_NSUP_ATTR_STATE => Ok(Self::AttributeStateNotSupported),
            VI_ERROR_ATTR_READONLY => Ok(Self::AttributeReadOnly),
            VI_ERROR_INV_LOCK_TYPE => Ok(Self::InvalidLockType),
            VI_ERROR_INV_ACCESS_KEY => Ok(Self::InvalidAccessKey),
            VI_ERROR_INV_EVENT => Ok(Self::InvalidEvent),
            VI_ERROR_INV_MECH => Ok(Self::InvalidMechanism),
            VI_ERROR_HNDLR_NINSTALLED => Ok(Self::HandlerNotInstalled),
            VI_ERROR_INV_HNDLR_REF => Ok(Self::InvalidHandlerReference),
            VI_ERROR_INV_CONTEXT => Ok(Self::InvalidContext),
            VI_ERROR_QUEUE_OVERFLOW => Ok(Self::QueueOverflow),
            VI_ERROR_NENABLED => Ok(Self::SessionNotEnabled),
            VI_ERROR_ABORT => Ok(Self::OperationAborted),
            VI_ERROR_RAW_WR_PROT_VIOL => Ok(Self::RawWriteProtocolViolation),
            VI_ERROR_RAW_RD_PROT_VIOL => Ok(Self::RawReadProtocolViolation),
            VI_ERROR_OUTP_PROT_VIOL => Ok(Self::OutputProtocolViolation),
            VI_ERROR_INP_PROT_VIOL => Ok(Self::InputProtocolViolation),
            VI_ERROR_BERR => Ok(Self::Bus),
            VI_ERROR_IN_PROGRESS => Ok(Self::OperationAlreadyInProgress),
            VI_ERROR_INV_SETUP => Ok(Self::InvalidSetup),
            VI_ERROR_QUEUE_ERROR => Ok(Self::Queue),
            VI_ERROR_ALLOC => Ok(Self::Allocation),
            VI_ERROR_INV_MASK => Ok(Self::InvalidBufferMask),
            VI_ERROR_IO => Ok(Self::Io),
            VI_ERROR_INV_FMT => Ok(Self::InvalidFormatSpecifier),
            VI_ERROR_NSUP_FMT => Ok(Self::FormatSpecifierNotSupported),
            VI_ERROR_LINE_IN_USE => Ok(Self::LineInUse),
            VI_ERROR_NSUP_MODE => Ok(Self::ModeNotSupported),
            VI_ERROR_SRQ_NOCCURRED => Ok(Self::ServiceRequestNotReceived),
            VI_ERROR_INV_SPACE => Ok(Self::InvalidAddressSpace),
            VI_ERROR_INV_OFFSET => Ok(Self::InvalidOffset),
            VI_ERROR_INV_WIDTH => Ok(Self::InvalidWidth),
            VI_ERROR_NSUP_OFFSET => Ok(Self::OffsetNotSupported),
            VI_ERROR_NSUP_VAR_WIDTH => Ok(Self::DifferentWidthNotSupported),
            VI_ERROR_WINDOW_NMAPPED => Ok(Self::SessionNotMapped),
            VI_ERROR_RESP_PENDING => Ok(Self::PreviousResponsePending),
            VI_ERROR_NLISTENERS => Ok(Self::NoListeners),
            VI_ERROR_NCIC => Ok(Self::NotControllerInCharge),
            VI_ERROR_NSYS_CNTLR => Ok(Self::NotSystemController),
            VI_ERROR_NSUP_OPER => Ok(Self::OperationNotSupported),
            VI_ERROR_INTR_PENDING => Ok(Self::PreviousInterruptPending),
            VI_ERROR_ASRL_PARITY => Ok(Self::Parity),
            VI_ERROR_ASRL_FRAMING => Ok(Self::Framing),
            VI_ERROR_ASRL_OVERRUN => Ok(Self::Overrun),
            VI_ERROR_TRIG_NMAPPED => Ok(Self::TriggerNotMapped),
            VI_ERROR_NSUP_ALIGN_OFFSET => Ok(Self::OffsetNotAligned),
            VI_ERROR_USER_BUF => Ok(Self::InvalidUserBuffer),
            VI_ERROR_RSRC_BUSY => Ok(Self::ResourceBusy),
            VI_ERROR_NSUP_WIDTH => Ok(Self::WidthNotSupported),
            VI_ERROR_INV_PARAMETER => Ok(Self::InvalidParameter),
            VI_ERROR_INV_PROT => Ok(Self::InvalidProtocol),
            VI_ERROR_INV_SIZE => Ok(Self::InvalidSize),
            VI_ERROR_WINDOW_MAPPED => Ok(Self::MappedWindow),
            VI_ERROR_NIMPL_OPER => Ok(Self::OperationNotImplemented),
            VI_ERROR_INV_LENGTH => Ok(Self::InvalidLength),
            VI_ERROR_INV_MODE => Ok(Self::InvalidMode),
            VI_ERROR_SESN_NLOCKED => Ok(Self::SessionNotLocked),
            VI_ERROR_MEM_NSHARED => Ok(Self::NoMemorySupport),
            VI_ERROR_LIBRARY_NFOUND => Ok(Self::LibraryNotFound),
            VI_ERROR_NSUP_INTR => Ok(Self::InterruptNotSupported),
            VI_ERROR_INV_LINE => Ok(Self::InvalidLine),
            VI_ERROR_FILE_ACCESS => Ok(Self::FileAccess),
            VI_ERROR_FILE_IO => Ok(Self::IoFile),
            VI_ERROR_NSUP_LINE => Ok(Self::LineNotSupported),
            VI_ERROR_NSUP_MECH => Ok(Self::MechanismNotSupported),
            VI_ERROR_INTF_NUM_NCONFIG => Ok(Self::InterfaceNumberNotConfigured),
            VI_ERROR_CONN_LOST => Ok(Self::ConnectionLost),
            VI_ERROR_MACHINE_NAVAIL => Ok(Self::MachineNotAvailable),
            VI_ERROR_NPERMISSION => Ok(Self::NoPermission),
            _ => Err(()),
        }
    }
}

impl TryFrom<ViStatus> for Status {
    type Error = VisaError;

    fn try_from(value: ViStatus) -> std::result::Result<Self, Self::Error> {
        match value as _ {
            VI_SUCCESS => Ok(Self::OperationCompleted),
            VI_SUCCESS_EVENT_EN => Ok(Self::EventAlreadyEnabled),
            VI_SUCCESS_EVENT_DIS => Ok(Self::EventAlreadyDisabled),
            VI_SUCCESS_QUEUE_EMPTY => Ok(Self::OperationCompletedQueueEmpty),
            VI_SUCCESS_TERM_CHAR => Ok(Self::TerminationCharacterRead),
            VI_SUCCESS_MAX_CNT => Ok(Self::BytesReadEqualInput),
            VI_WARN_QUEUE_OVERFLOW => Ok(Self::QueueOverflow),
            VI_WARN_CONFIG_NLOADED => Ok(Self::ConfigurationNotLoaded),
            VI_SUCCESS_DEV_NPRESENT => Ok(Self::DeviceNotResponding),
            VI_SUCCESS_TRIG_MAPPED => Ok(Self::TriggerAlreadyMapped),
            VI_SUCCESS_QUEUE_NEMPTY => Ok(Self::QueueNotEmpty),
            VI_WARN_NULL_OBJECT => Ok(Self::NullObject),
            VI_WARN_NSUP_ATTR_STATE => Ok(Self::AttributeStateNotSupported),
            VI_WARN_UNKNOWN_STATUS => Ok(Self::Unknown),
            VI_WARN_NSUP_BUF => Ok(Self::BufferNotSupported),
            VI_SUCCESS_NCHAIN => Ok(Self::DoNotInvokeEvent),
            VI_SUCCESS_NESTED_SHARED => Ok(Self::NestedSharedLocks),
            VI_SUCCESS_NESTED_EXCLUSIVE => Ok(Self::NestedExclusiveLocks),
            VI_SUCCESS_SYNC => Ok(Self::AsynchronousOperationHandledSynchronously),
            VI_WARN_EXT_FUNC_NIMPL => Ok(Self::ExtendedFunctionNotImplemented),
            error => Err(VisaError::try_from(error as i32).unwrap()),
        }
    }
}

impl VisaError {
    pub fn to_io_error(self) -> std::io::Error {
        use std::io::Error;
        use std::io::ErrorKind;
        let kind = match self {
            Self::InvalidObject => ErrorKind::AddrNotAvailable,
            Self::OperationNotSupported => ErrorKind::Unsupported,
            Self::ResourceLocked => ErrorKind::ConnectionRefused,
            Self::Timeout => ErrorKind::TimedOut,
            Self::RawWriteProtocolViolation | Self::RawReadProtocolViolation => {
                ErrorKind::InvalidData
            }
            Self::InputProtocolViolation | Self::OutputProtocolViolation | Self::Bus => {
                ErrorKind::BrokenPipe
            }
            Self::InvalidSetup | Self::InvalidBufferMask => ErrorKind::InvalidInput,
            Self::NotControllerInCharge => ErrorKind::PermissionDenied,
            Self::NoListeners | Self::Parity | Self::Framing | Self::Overrun => ErrorKind::Other,
            Self::ConnectionLost => ErrorKind::BrokenPipe,
            Self::Io => std::io::Error::last_os_error().kind(),
            e => ErrorKind::Other,
        };

        Error::new(kind, self)
    }
}

pub fn parse_vi_status(status: ViStatus) -> Result<Status> {
    Status::try_from(status).map_err(|error| Error::Visa(error))
}

pub fn parse_vi_status_to_io(status: ViStatus) -> std::io::Result<Status> {
    Status::try_from(status).map_err(|error| error.to_io_error())
}
