import com.amazonaws.services.lambda.runtime.Context;
import com.amazonaws.services.lambda.runtime.RequestHandler;
import com.code.to.the.moon.MemorizeThisDB;

public class MemorizeThisDBLambdaHelper implements RequestHandler<Object, String> {
    @Override
    public String handleRequest(Object input, Context context) {
        return new MemorizeThisDB().handleRequest(input, context);
    }
}
