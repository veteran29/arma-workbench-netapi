//#ifdef WORKBENCH

class FileSystemApiResponse : JsonApiStruct
{
	bool banana = "uga buga";

	void FileSystemApiResponse()
	{
		RegV("banana");
	}
}

class FileSystemApi: NetApiHandler
{
	//override JsonApiStruct GetRequest()
	//{
	//}
	
	override JsonApiStruct GetResponse(JsonApiStruct request)
	{
		FileSystemApiResponse response = new FileSystemApiResponse();
	
		return response;
	}
}

//#endif // WORKBENCH
