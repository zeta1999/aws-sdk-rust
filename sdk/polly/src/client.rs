// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

#[derive(std::fmt::Debug)]
pub(crate) struct Handle {
    client: aws_hyper::Client<aws_hyper::conn::Standard>,
    conf: crate::Config,
}

#[derive(Clone, std::fmt::Debug)]
pub struct Client {
    handle: std::sync::Arc<Handle>,
}

impl Client {
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_env() -> Self {
        Self::from_conf_conn(
            crate::Config::builder().build(),
            aws_hyper::conn::Standard::https(),
        )
    }

    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        Self::from_conf_conn(conf, aws_hyper::conn::Standard::https())
    }

    pub fn from_conf_conn(conf: crate::Config, conn: aws_hyper::conn::Standard) -> Self {
        let client = aws_hyper::Client::new(conn);
        Self {
            handle: std::sync::Arc::new(Handle { conf, client }),
        }
    }

    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }

    pub fn delete_lexicon(&self) -> fluent_builders::DeleteLexicon {
        fluent_builders::DeleteLexicon::new(self.handle.clone())
    }

    pub fn describe_voices(&self) -> fluent_builders::DescribeVoices {
        fluent_builders::DescribeVoices::new(self.handle.clone())
    }

    pub fn get_lexicon(&self) -> fluent_builders::GetLexicon {
        fluent_builders::GetLexicon::new(self.handle.clone())
    }

    pub fn get_speech_synthesis_task(&self) -> fluent_builders::GetSpeechSynthesisTask {
        fluent_builders::GetSpeechSynthesisTask::new(self.handle.clone())
    }

    pub fn list_lexicons(&self) -> fluent_builders::ListLexicons {
        fluent_builders::ListLexicons::new(self.handle.clone())
    }

    pub fn list_speech_synthesis_tasks(&self) -> fluent_builders::ListSpeechSynthesisTasks {
        fluent_builders::ListSpeechSynthesisTasks::new(self.handle.clone())
    }

    pub fn put_lexicon(&self) -> fluent_builders::PutLexicon {
        fluent_builders::PutLexicon::new(self.handle.clone())
    }

    pub fn start_speech_synthesis_task(&self) -> fluent_builders::StartSpeechSynthesisTask {
        fluent_builders::StartSpeechSynthesisTask::new(self.handle.clone())
    }

    pub fn synthesize_speech(&self) -> fluent_builders::SynthesizeSpeech {
        fluent_builders::SynthesizeSpeech::new(self.handle.clone())
    }
}
pub mod fluent_builders {

    pub struct DeleteLexicon {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::delete_lexicon_input::Builder,
    }
    impl DeleteLexicon {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> Result<
            crate::output::DeleteLexiconOutput,
            smithy_http::result::SdkError<crate::error::DeleteLexiconError>,
        > {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }

        /// <p>The name of the lexicon to delete. Must be an existing lexicon in the region.</p>
        pub fn name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.name(inp);
            self
        }
        pub fn set_name(mut self, inp: std::string::String) -> Self {
            self.inner = self.inner.set_name(inp);
            self
        }
    }

    pub struct DescribeVoices {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::describe_voices_input::Builder,
    }
    impl DescribeVoices {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> Result<
            crate::output::DescribeVoicesOutput,
            smithy_http::result::SdkError<crate::error::DescribeVoicesError>,
        > {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }

        /// <p>Specifies the engine (<code>standard</code> or <code>neural</code>) used by Amazon Polly
        /// when processing input text for speech synthesis. </p>
        pub fn engine(mut self, inp: crate::model::Engine) -> Self {
            self.inner = self.inner.engine(inp);
            self
        }
        pub fn set_engine(mut self, inp: std::option::Option<crate::model::Engine>) -> Self {
            self.inner = self.inner.set_engine(inp);
            self
        }
        /// <p> The language identification tag (ISO 639 code for the language name-ISO 3166 country
        /// code) for filtering the list of voices returned. If you don't specify this optional parameter,
        /// all available voices are returned. </p>
        pub fn language_code(mut self, inp: crate::model::LanguageCode) -> Self {
            self.inner = self.inner.language_code(inp);
            self
        }
        pub fn set_language_code(
            mut self,
            inp: std::option::Option<crate::model::LanguageCode>,
        ) -> Self {
            self.inner = self.inner.set_language_code(inp);
            self
        }
        /// <p>Boolean value indicating whether to return any bilingual voices that use the specified
        /// language as an additional language. For instance, if you request all languages that use US
        /// English (es-US), and there is an Italian voice that speaks both Italian (it-IT) and US
        /// English, that voice will be included if you specify <code>yes</code> but not if you specify
        /// <code>no</code>.</p>
        pub fn include_additional_language_codes(mut self, inp: bool) -> Self {
            self.inner = self.inner.include_additional_language_codes(inp);
            self
        }
        pub fn set_include_additional_language_codes(mut self, inp: bool) -> Self {
            self.inner = self.inner.set_include_additional_language_codes(inp);
            self
        }
        /// <p>An opaque pagination token returned from the previous <code>DescribeVoices</code>
        /// operation. If present, this indicates where to continue the listing.</p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        pub fn set_next_token(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(inp);
            self
        }
    }

    pub struct GetLexicon {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::get_lexicon_input::Builder,
    }
    impl GetLexicon {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> Result<
            crate::output::GetLexiconOutput,
            smithy_http::result::SdkError<crate::error::GetLexiconError>,
        > {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }

        /// <p>Name of the lexicon.</p>
        pub fn name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.name(inp);
            self
        }
        pub fn set_name(mut self, inp: std::string::String) -> Self {
            self.inner = self.inner.set_name(inp);
            self
        }
    }

    pub struct GetSpeechSynthesisTask {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::get_speech_synthesis_task_input::Builder,
    }
    impl GetSpeechSynthesisTask {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> Result<
            crate::output::GetSpeechSynthesisTaskOutput,
            smithy_http::result::SdkError<crate::error::GetSpeechSynthesisTaskError>,
        > {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }

        /// <p>The Amazon Polly generated identifier for a speech synthesis task.</p>
        pub fn task_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.task_id(inp);
            self
        }
        pub fn set_task_id(mut self, inp: std::string::String) -> Self {
            self.inner = self.inner.set_task_id(inp);
            self
        }
    }

    pub struct ListLexicons {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::list_lexicons_input::Builder,
    }
    impl ListLexicons {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> Result<
            crate::output::ListLexiconsOutput,
            smithy_http::result::SdkError<crate::error::ListLexiconsError>,
        > {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }

        /// <p>An opaque pagination token returned from previous <code>ListLexicons</code> operation.
        /// If present, indicates where to continue the list of lexicons.</p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        pub fn set_next_token(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(inp);
            self
        }
    }

    pub struct ListSpeechSynthesisTasks {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::list_speech_synthesis_tasks_input::Builder,
    }
    impl ListSpeechSynthesisTasks {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> Result<
            crate::output::ListSpeechSynthesisTasksOutput,
            smithy_http::result::SdkError<crate::error::ListSpeechSynthesisTasksError>,
        > {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }

        /// <p>Maximum number of speech synthesis tasks returned in a List operation.</p>
        pub fn max_results(mut self, inp: i32) -> Self {
            self.inner = self.inner.max_results(inp);
            self
        }
        pub fn set_max_results(mut self, inp: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(inp);
            self
        }
        /// <p>The pagination token to use in the next request to continue the listing of speech
        /// synthesis tasks. </p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        pub fn set_next_token(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(inp);
            self
        }
        /// <p>Status of the speech synthesis tasks returned in a List operation</p>
        pub fn status(mut self, inp: crate::model::TaskStatus) -> Self {
            self.inner = self.inner.status(inp);
            self
        }
        pub fn set_status(mut self, inp: std::option::Option<crate::model::TaskStatus>) -> Self {
            self.inner = self.inner.set_status(inp);
            self
        }
    }

    pub struct PutLexicon {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::put_lexicon_input::Builder,
    }
    impl PutLexicon {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> Result<
            crate::output::PutLexiconOutput,
            smithy_http::result::SdkError<crate::error::PutLexiconError>,
        > {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }

        /// <p>Name of the lexicon. The name must follow the regular express format [0-9A-Za-z]{1,20}.
        /// That is, the name is a case-sensitive alphanumeric string up to 20 characters long. </p>
        pub fn name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.name(inp);
            self
        }
        pub fn set_name(mut self, inp: std::string::String) -> Self {
            self.inner = self.inner.set_name(inp);
            self
        }
        /// <p>Content of the PLS lexicon as string data.</p>
        pub fn content(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.content(inp);
            self
        }
        pub fn set_content(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_content(inp);
            self
        }
    }

    pub struct StartSpeechSynthesisTask {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::start_speech_synthesis_task_input::Builder,
    }
    impl StartSpeechSynthesisTask {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> Result<
            crate::output::StartSpeechSynthesisTaskOutput,
            smithy_http::result::SdkError<crate::error::StartSpeechSynthesisTaskError>,
        > {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }

        /// <p>Specifies the engine (<code>standard</code> or <code>neural</code>) for Amazon Polly to
        /// use when processing input text for speech synthesis. Using a voice that is not supported for
        /// the engine selected will result in an error.</p>
        pub fn engine(mut self, inp: crate::model::Engine) -> Self {
            self.inner = self.inner.engine(inp);
            self
        }
        pub fn set_engine(mut self, inp: std::option::Option<crate::model::Engine>) -> Self {
            self.inner = self.inner.set_engine(inp);
            self
        }
        /// <p>Optional language code for the Speech Synthesis request. This is only necessary if using a
        /// bilingual voice, such as Aditi, which can be used for either Indian English (en-IN) or Hindi
        /// (hi-IN). </p>
        /// <p>If a bilingual voice is used and no language code is specified, Amazon Polly will use the
        /// default language of the bilingual voice. The default language for any voice is the one
        /// returned by the <a href="https://docs.aws.amazon.com/polly/latest/dg/API_DescribeVoices.html">DescribeVoices</a> operation for the <code>LanguageCode</code> parameter. For example,
        /// if no language code is specified, Aditi will use Indian English rather than Hindi.</p>
        pub fn language_code(mut self, inp: crate::model::LanguageCode) -> Self {
            self.inner = self.inner.language_code(inp);
            self
        }
        pub fn set_language_code(
            mut self,
            inp: std::option::Option<crate::model::LanguageCode>,
        ) -> Self {
            self.inner = self.inner.set_language_code(inp);
            self
        }
        /// <p>List of one or more pronunciation lexicon names you want the service to apply during
        /// synthesis. Lexicons are applied only if the language of the lexicon is the same as the
        /// language of the voice. </p>
        pub fn lexicon_names(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.lexicon_names(inp);
            self
        }
        pub fn set_lexicon_names(
            mut self,
            inp: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_lexicon_names(inp);
            self
        }
        /// <p>The format in which the returned output will be encoded. For audio stream, this will be
        /// mp3, ogg_vorbis, or pcm. For speech marks, this will be json. </p>
        pub fn output_format(mut self, inp: crate::model::OutputFormat) -> Self {
            self.inner = self.inner.output_format(inp);
            self
        }
        pub fn set_output_format(
            mut self,
            inp: std::option::Option<crate::model::OutputFormat>,
        ) -> Self {
            self.inner = self.inner.set_output_format(inp);
            self
        }
        /// <p>Amazon S3 bucket name to which the output file will be saved.</p>
        pub fn output_s3_bucket_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.output_s3_bucket_name(inp);
            self
        }
        pub fn set_output_s3_bucket_name(
            mut self,
            inp: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_output_s3_bucket_name(inp);
            self
        }
        /// <p>The Amazon S3 key prefix for the output speech file.</p>
        pub fn output_s3_key_prefix(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.output_s3_key_prefix(inp);
            self
        }
        pub fn set_output_s3_key_prefix(
            mut self,
            inp: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_output_s3_key_prefix(inp);
            self
        }
        /// <p>The audio frequency specified in Hz.</p>
        /// <p>The valid values for mp3 and ogg_vorbis are "8000", "16000", "22050", and "24000". The
        /// default value for standard voices is "22050". The default value for neural voices is
        /// "24000".</p>
        /// <p>Valid values for pcm are "8000" and "16000" The default value is "16000". </p>
        pub fn sample_rate(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.sample_rate(inp);
            self
        }
        pub fn set_sample_rate(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_sample_rate(inp);
            self
        }
        /// <p>ARN for the SNS topic optionally used for providing status notification for a speech
        /// synthesis task.</p>
        pub fn sns_topic_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.sns_topic_arn(inp);
            self
        }
        pub fn set_sns_topic_arn(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_sns_topic_arn(inp);
            self
        }
        /// <p>The type of speech marks returned for the input text.</p>
        pub fn speech_mark_types(mut self, inp: impl Into<crate::model::SpeechMarkType>) -> Self {
            self.inner = self.inner.speech_mark_types(inp);
            self
        }
        pub fn set_speech_mark_types(
            mut self,
            inp: std::option::Option<std::vec::Vec<crate::model::SpeechMarkType>>,
        ) -> Self {
            self.inner = self.inner.set_speech_mark_types(inp);
            self
        }
        /// <p>The input text to synthesize. If you specify ssml as the TextType, follow the SSML format
        /// for the input text. </p>
        pub fn text(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.text(inp);
            self
        }
        pub fn set_text(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_text(inp);
            self
        }
        /// <p>Specifies whether the input text is plain text or SSML. The default value is plain text.
        /// </p>
        pub fn text_type(mut self, inp: crate::model::TextType) -> Self {
            self.inner = self.inner.text_type(inp);
            self
        }
        pub fn set_text_type(mut self, inp: std::option::Option<crate::model::TextType>) -> Self {
            self.inner = self.inner.set_text_type(inp);
            self
        }
        /// <p>Voice ID to use for the synthesis. </p>
        pub fn voice_id(mut self, inp: crate::model::VoiceId) -> Self {
            self.inner = self.inner.voice_id(inp);
            self
        }
        pub fn set_voice_id(mut self, inp: std::option::Option<crate::model::VoiceId>) -> Self {
            self.inner = self.inner.set_voice_id(inp);
            self
        }
    }

    pub struct SynthesizeSpeech {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::synthesize_speech_input::Builder,
    }
    impl SynthesizeSpeech {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> Result<
            crate::output::SynthesizeSpeechOutput,
            smithy_http::result::SdkError<crate::error::SynthesizeSpeechError>,
        > {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }

        /// <p>Specifies the engine (<code>standard</code> or <code>neural</code>) for Amazon Polly to
        /// use when processing input text for speech synthesis. For information on Amazon Polly voices and which voices are available in standard-only, NTTS-only, and
        /// both standard and NTTS formats, see <a href="https://docs.aws.amazon.com/polly/latest/dg/voicelist.html">Available Voices</a>.</p>
        /// <p>
        /// <b>NTTS-only voices</b>
        /// </p>
        /// <p>When using NTTS-only voices such as Kevin (en-US), this parameter is required and must be
        /// set to <code>neural</code>. If the engine is not specified, or is set to <code>standard</code>,
        /// this will result in an error. </p>
        /// <p>Type: String</p>
        /// <p>Valid Values: <code>standard</code>  |  <code>neural</code>
        /// </p>
        /// <p>Required: Yes</p>
        /// <p>
        /// <b>Standard voices</b>
        /// </p>
        /// <p>For standard voices, this is not required; the engine parameter defaults to
        /// <code>standard</code>. If the engine is not specified, or is set to <code>standard</code> and
        /// an NTTS-only voice is selected, this will result in an error. </p>
        pub fn engine(mut self, inp: crate::model::Engine) -> Self {
            self.inner = self.inner.engine(inp);
            self
        }
        pub fn set_engine(mut self, inp: std::option::Option<crate::model::Engine>) -> Self {
            self.inner = self.inner.set_engine(inp);
            self
        }
        /// <p>Optional language code for the Synthesize Speech request. This is only necessary if using
        /// a bilingual voice, such as Aditi, which can be used for either Indian English (en-IN) or Hindi
        /// (hi-IN). </p>
        /// <p>If a bilingual voice is used and no language code is specified, Amazon Polly will use the
        /// default language of the bilingual voice. The default language for any voice is the one
        /// returned by the <a href="https://docs.aws.amazon.com/polly/latest/dg/API_DescribeVoices.html">DescribeVoices</a> operation for the <code>LanguageCode</code> parameter. For example,
        /// if no language code is specified, Aditi will use Indian English rather than Hindi.</p>
        pub fn language_code(mut self, inp: crate::model::LanguageCode) -> Self {
            self.inner = self.inner.language_code(inp);
            self
        }
        pub fn set_language_code(
            mut self,
            inp: std::option::Option<crate::model::LanguageCode>,
        ) -> Self {
            self.inner = self.inner.set_language_code(inp);
            self
        }
        /// <p>List of one or more pronunciation lexicon names you want the service to apply during
        /// synthesis. Lexicons are applied only if the language of the lexicon is the same as the
        /// language of the voice. For information about storing lexicons, see <a href="https://docs.aws.amazon.com/polly/latest/dg/API_PutLexicon.html">PutLexicon</a>.</p>
        pub fn lexicon_names(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.lexicon_names(inp);
            self
        }
        pub fn set_lexicon_names(
            mut self,
            inp: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_lexicon_names(inp);
            self
        }
        /// <p> The format in which the returned output will be encoded. For audio stream, this will
        /// be mp3, ogg_vorbis, or pcm. For speech marks, this will be json. </p>
        /// <p>When pcm is used, the content returned is audio/pcm in a signed 16-bit, 1 channel
        /// (mono), little-endian format. </p>
        pub fn output_format(mut self, inp: crate::model::OutputFormat) -> Self {
            self.inner = self.inner.output_format(inp);
            self
        }
        pub fn set_output_format(
            mut self,
            inp: std::option::Option<crate::model::OutputFormat>,
        ) -> Self {
            self.inner = self.inner.set_output_format(inp);
            self
        }
        /// <p>The audio frequency specified in Hz.</p>
        /// <p>The valid values for mp3 and ogg_vorbis are "8000", "16000", "22050", and "24000". The
        /// default value for standard voices is "22050". The default value for neural voices is
        /// "24000".</p>
        /// <p>Valid values for pcm are "8000" and "16000" The default value is "16000". </p>
        pub fn sample_rate(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.sample_rate(inp);
            self
        }
        pub fn set_sample_rate(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_sample_rate(inp);
            self
        }
        /// <p>The type of speech marks returned for the input text.</p>
        pub fn speech_mark_types(mut self, inp: impl Into<crate::model::SpeechMarkType>) -> Self {
            self.inner = self.inner.speech_mark_types(inp);
            self
        }
        pub fn set_speech_mark_types(
            mut self,
            inp: std::option::Option<std::vec::Vec<crate::model::SpeechMarkType>>,
        ) -> Self {
            self.inner = self.inner.set_speech_mark_types(inp);
            self
        }
        /// <p> Input text to synthesize. If you specify <code>ssml</code> as the
        /// <code>TextType</code>, follow the SSML format for the input text. </p>
        pub fn text(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.text(inp);
            self
        }
        pub fn set_text(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_text(inp);
            self
        }
        /// <p> Specifies whether the input text is plain text or SSML. The default value is plain
        /// text. For more information, see <a href="https://docs.aws.amazon.com/polly/latest/dg/ssml.html">Using SSML</a>.</p>
        pub fn text_type(mut self, inp: crate::model::TextType) -> Self {
            self.inner = self.inner.text_type(inp);
            self
        }
        pub fn set_text_type(mut self, inp: std::option::Option<crate::model::TextType>) -> Self {
            self.inner = self.inner.set_text_type(inp);
            self
        }
        /// <p> Voice ID to use for the synthesis. You can get a list of available voice IDs by
        /// calling the <a href="https://docs.aws.amazon.com/polly/latest/dg/API_DescribeVoices.html">DescribeVoices</a> operation. </p>
        pub fn voice_id(mut self, inp: crate::model::VoiceId) -> Self {
            self.inner = self.inner.voice_id(inp);
            self
        }
        pub fn set_voice_id(mut self, inp: std::option::Option<crate::model::VoiceId>) -> Self {
            self.inner = self.inner.set_voice_id(inp);
            self
        }
    }
}
