#[derive(Copy, Clone, Debug)]
pub enum Attribute {
    Printer(PrinterAttribute),
}

#[derive(Copy, Clone, Debug)]
pub enum PrinterAttribute {
    // IPP/1.1 Attributes
    CharsetConfigured,
    CharsetSupported,
    CompressionSupported,
    DocumentFormatDefault,
    DocumentFormatSupported,
    GeneratedNaturalLanguageSupported,
    IppVersionsSupported,
    NaturalLanguageConfigured,
    OperationsSupported,
    PdlOverrideSupported,
    PrinterIsAcceptingJobs,
    PrinterName,
    PrinterState,
    PrinterStateReasons,
    PrinterUpTime,
    PrinterUriSupported,
    QueuedJobCount,
    UriAuthenticationSupported,
    UriSecuritySupported,
    // // IPP/2.0 Attributes
    // ColorSupported,
    // CopiesDefault,
    // CopiesSupported,
    // FinishingsDefault,
    // FinishingsSupported,
    // MediaDefault,
    // MediaReady,
    // MediaSupported,
    // OrientationRequestedDefault,
    // OrientationRequestedSupported,
    // OutputBinDefault,
    // OutputBinSupported,
    // PagesPerMinute,
    // PagesPerMinuteColor,
    // PrintQualityDefault,
    // PrintQualitySupported,
    // PrinterInfo,
    // PrinterLocation,
    // PrinterMakeAndModel,
    // PrinterMoreInfo,
    // PrinterResolutionDefault,
    // PrinterResolutionSupported,
    // SidesDefault,
    // SidesSupported,
    // // IPP/2.1 Attributes
    // IppgetEventLife,
    // JobHoldUntilDefault,
    // JobHoldUntilSupported,
    // JobPriorityDefault,
    // JobPrioritySupported,
    // JobSettableAttributesSupported,
    // JobSheetsDefault,
    // JobSheetsSupported,
    // MediaColDefault,
    // MediaColReady,
    // MediaColSupported,
    // MultipleOperationTimeOut,
    // NotifyEventsDefault,
    // NotifyEventsSupported,
    // NotifyLeaseDurationDefault,
    // NotifyLeaseDurationSupported,
    // NotifyMaxEventsSupported,
    // NotifyPullMethodSupported,
    // OverridesSupported,
    // PrinterAlert,
    // PrinterAlertDescription,
    // PrinterSettableAttributesSupported,
    // PrinterStateChangeTime,
    // WhichJobsSupported,
    // // IPP/2.2 Attributes
    // JobCreationAttributesSupported,
    // JobIdsSupported,
    // MultipleDocumentJobsSupported,
    // NumberUpDefault,
    // NumberUpSupported,
    // PageRangesSupported,
    // PrinterDeviceId,
    // PrinterMessageFromOperator,
}
