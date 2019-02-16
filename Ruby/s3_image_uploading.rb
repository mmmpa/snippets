module S3ImageUploading
  extend ActiveSupport::Concern

  DATA_URL_HEADER_PICK = %r{\Adata:([-\w]+\/[-\w\+\.]+)?;base64,(.*)}m.freeze
  EXTENSION_PICK = %r{\A(image/)(.+)}.freeze
  EXTENSIONS = Set.new(%w[png jpg jpeg gif])

  module ClassMethods
    def bucket
      Aws::S3::Resource.
        new(region: ENV['AWS_DEFAULT_REGION']).
        bucket(ENV['AWS_IMAGE_BUCKET_NAME'])
    end

    def upload_data_urls(data_urls = [])
      split_urls = data_urls.map do |data_url|
        _, type, data = data_url.match(DATA_URL_HEADER_PICK).to_a

        raise NotImage if !type || !data

        _, _, extension = type.match(EXTENSION_PICK).to_a

        raise NotImage unless EXTENSIONS.include?(extension)

        [type, data, extension]
      end

      split_urls.map do |type, data, extension|
        result = bucket.put_object(
          acl: 'public-read',
          body: Base64.decode64(data),
          content_encoding: 'base64',
          content_length: 1,
          content_type: type,
          expires: Time.zone.now,
          key: "#{key}.#{extension}",
        )

        "#{ENV["AWS_IMAGE_URL"]}/#{result.key}"
      end
    end

    def key(now = Time.zone.now)
      [10_000_000_000 - now.to_i, SecureRandom.uuid, SecureRandom.uuid].join('-')
    end
  end

  def upload_data_urls(data_urls = [])
    self.class.upload_data_urls(data_urls)
  end

  class NotImage < StandardError
  end
end
